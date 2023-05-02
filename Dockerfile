# create base
FROM rust as base
WORKDIR /app
RUN cargo install cargo-chef

# stage 1
FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# stage 2 - 1
FROM base as cacher_back
COPY --from=planner /app/recipe.json /app/recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# stage 2 - 2
FROM base as cacher_front

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

COPY --from=planner /app/recipe.json /app/recipe.json
RUN cargo chef cook --release -p front_schedule_leptos --target wasm32-unknown-unknown

# stage 3
FROM rust as builder

ENV USER=web
ENV UID=1001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


# change the working directory
WORKDIR /app

# stage 3 - 1
# use the official rust image
FROM builder as backend_builder

# copy the backend app
COPY . /app

# copy deps
COPY --from=cacher_back /app/target target
COPY --from=cacher_back /usr/local/cargo /usr/local/cargo

# build the app
RUN cargo build --release

# stage 3 - 2
# use the official rust image
FROM builder as frontend_builder

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk 

# copy the backend app
COPY . /app

# copy deps
COPY --from=cacher_front /app/target target
COPY --from=cacher_front /usr/local/cargo /usr/local/cargo

# build the frontend
RUN cd frontend && trunk build --release

# stage 4
# run the application in smaller container
FROM gcr.io/distroless/cc-debian11

# Import from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# copy the release build
COPY --from=backend_builder /app/target/release/backend /app/backend
COPY --from=frontend_builder /app/frontend/dist /app/dist
WORKDIR /app

USER web:web

# start the application
CMD ["./backend"]
