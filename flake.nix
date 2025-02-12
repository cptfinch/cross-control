{
  description = "Rust Barrier - Event System";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        nativeBuildInputs = with pkgs; [ 
          pkg-config
          clang
        ];

        buildInputs = with pkgs; [
          # X11 dependencies
          xorg.libX11
          xorg.libxcb
          libxkbcommon
          xkeyboard_config
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.xorg.libX11
            pkgs.xorg.libxcb
            pkgs.libxkbcommon
          ];
          
          XKB_CONFIG_ROOT = "${pkgs.xkeyboard_config}/etc/X11/xkb";
          
          packages = [ rustToolchain ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          shellHook = ''
            echo "Rust+X11 development environment loaded!"
            echo "XKB config path: $XKB_CONFIG_ROOT"
          '';
        };
      }
    );
} 