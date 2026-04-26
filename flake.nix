{
  description = "serde Serializer + Deserializer for the nexus data format";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rustfmt"
          "clippy"
          "rust-analyzer"
          "rust-src"
        ];
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "nexus-serde";
          packages = [
            pkgs.jujutsu
            pkgs.pkg-config
            toolchain
          ];
        };

        # `nix build` produces the library; `nix flake check` runs the
        # same derivation with doCheck=true, exercising `cargo test` in
        # a pure sandbox — no host-toolchain drift.
        packages.default = rustPlatform.buildRustPackage {
          pname = "nexus-serde";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            # Hash of the nota-serde-core git checkout Cargo pins in
            # Cargo.lock. Bump when the locked rev changes.
            outputHashes = {
              "nota-serde-core-0.1.0" = "sha256-3Z3Ly1yBuQmEJi1lS1b36hgqM4mpbBg8Fh2rQ4qXUuI=";
            };
          };
          doCheck = true;
        };

        checks.default = self.packages.${system}.default;
      }
    );
}
