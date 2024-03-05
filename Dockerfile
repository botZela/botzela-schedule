# create base
FROM rust:bullseye as base
# RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
RUN cargo install cargo-chef

# stage 1
FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# stage 2 
FROM base as cacher
COPY --from=planner /app/recipe.json /app/recipe.json
RUN cargo chef cook --release --recipe-path recipe.json && \
  cargo chef cook --release -p front_schedule_leptos --target wasm32-unknown-unknown

# stage 3
FROM rust:bullseye as builder

# RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk 
RUN cargo install wasm-bindgen-cli

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
# copy the backend app
COPY . /app

# copy deps
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# build the app
ARG CLIENT_DIST="/app/dist"
RUN cargo build --release
RUN cd frontend && trunk build --release

# stage 4
# run the application in smaller container
FROM gcr.io/distroless/cc-debian11

# Import from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# copy the release build
COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/frontend/dist /app/dist
WORKDIR /app

USER web:web

# start the application
ARG CLIENT_DIST=/app/dist
CMD ["./backend"]
