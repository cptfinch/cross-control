{
  description = "Rust Barrier - Event System";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        nativeBuildInputs = with pkgs; [
          rustToolchain
          cargo-watch
          cargo-tarpaulin
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
          libiconv
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          shellHook = ''
            echo "Rust development environment loaded!"
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo test     - Run tests"
            echo "  cargo watch -x test  - Run tests on file changes"
            echo "  cargo tarpaulin    - Generate test coverage"
          '';
        };
      }
    );
} 