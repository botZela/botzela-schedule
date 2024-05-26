{
  description = "BotZela Schedule Web APP";

  inputs = {
    systems.url = "github:nix-systems/default-linux";

    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.systems.follows = "systems";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    pre-commit-hooks,
    ...
  } @ inputs:
    {
      hydraJobs = {
        inherit (self) packages;
      };
    }
    // flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;

      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      # When filtering sources, we want to allow assets other than .rs files
      src = lib.cleanSourceWith {
        src = craneLib.path ./.; # The original, unfiltered source
        filter = path: type:
          (lib.hasSuffix "\.html" path)
          || (lib.hasSuffix "\.lock" path)
          || (lib.hasSuffix "\.scss" path)
          || (lib.hasSuffix "\.css" path)
          ||
          # Example of a folder for images, icons, etc
          (lib.hasInfix "/assets/" path)
          || (lib.hasInfix "/public/" path)
          ||
          # Default filter from crane (allow .rs files)
          (craneLib.filterCargoSources path type);
      };
      # src = craneLib.cleanCargoSource (craneLib.path ./.);

      # Arguments to be used by both the frontend and the server
      # When building a workspace with crane, it's a good idea
      # to set "pname" and "version".
      commonArgs = {
        inherit src;
        pname = "botzela-schedule";
        version = "0.1.0";
      };

      # Native packages

      nativeArgs =
        commonArgs
        // {
          pname = "backend";
        };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

      # Simple JSON API that can be queried by the frontend
      backend = craneLib.buildPackage (nativeArgs
        // {
          inherit cargoArtifacts;
          CLIENT_DIST = frontend;
        });

      # Wasm packages

      # it's not possible to build the server on the
      # wasm32 target, so we only build the frontend.
      wasmArgs =
        commonArgs
        // {
          pname = "botzela-schedule-wasm";
          cargoExtraArgs = "--package=front_schedule_leptos";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

      cargoArtifactsWasm = craneLib.buildDepsOnly (wasmArgs
        // {
          doCheck = false;
        });

      # Build the frontend of the application.
      # This derivation is a directory you can put on a webserver.
      frontend = craneLib.buildTrunkPackage (wasmArgs
        // {
          pname = "front_schedule_leptos";
          cargoArtifacts = cargoArtifactsWasm;
          trunkIndexPath = "frontend/index.html";
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
            version = "0.2.92";
            hash = "sha256-1VwY8vQy7soKEgbki4LD+v259751kKxSxmo/gqE6yV0=";
            cargoHash = "sha256-aACJ+lYNEU8FFBs158G1/JG8sc6Rq080PeKCMnwdpH0=";
          };
        });

      # Create Docker Image with Nix
      dockerImage = pkgs.dockerTools.streamLayeredImage {
        name = "botzela-schedule-nix";
        tag = "latest";
        config = {
          Cmd = ["${backend}/bin/backend"];
          Env = ["CLIENT_DIST=${frontend}"];
        };
      };
    in {
      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            alejandra.enable = true;
            statix.enable = true;
            typos.enable = true;
          };
        };
        # Build the crate as part of `nix flake check` for convenience
        inherit backend frontend;

        # Run clippy (and deny all warnings) on the crate source,
        # again, reusing the dependency artifacts from above.
        #
        # Note that this is done as a separate derivation so that
        # we can block the CI if there are issues here, but not
        # prevent downstream consumers from building our crate by itself.
        my-app-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            # Here we don't care about serving the frontend
            CLIENT_DIST = "";
          });

        # Check formatting
        my-app-fmt = craneLib.cargoFmt commonArgs;
      };

      apps.default = flake-utils.lib.mkApp {
        name = "backend";
        drv = backend;
      };

      packages = {
        # that way we can build `bin` specifically,
        # but it's also the default.
        inherit backend dockerImage;
        default = backend;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        shellHook =
          ''
            export CLIENT_DIST=$PWD/frontend/dist;
          ''
          + self.checks.${system}.pre-commit-check.shellHook;

        buildInputs = self.checks.${system}.pre-commit-check.enabledPackages;

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          pkg-config
          trunk
          rustToolchain
          dive
          flyctl
          just
          cargo-edit
          cargo-audit
          rust-analyzer
          leptosfmt
        ];
      };
    });
}
