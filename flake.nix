{
  description = "ferric - A TUI for burning bootable USB drives";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
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
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        runtimeDeps = with pkgs; [
          util-linux  # provides lsblk
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          packages = with pkgs; [
            cargo-watch
            cargo-edit
            rust-analyzer
            rustfmt
            clippy
          ] ++ runtimeDeps;

          shellHook = ''
            echo "🦀 ferric development environment"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo run            - Run ferric (requires real TTY)"
            echo "  cargo test           - Run tests"
            echo "  cargo watch          - Watch for changes and rebuild"
            echo ""
            echo "Note: Running ferric requires root privileges for writing to devices."
            echo "      Use: sudo -E cargo run"
            echo ""
            rustc --version
            cargo --version
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "ferric";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs ++ runtimeDeps;

          postInstall = ''
            wrapProgram $out/bin/ferric \
              --prefix PATH : ${pkgs.lib.makeBinPath runtimeDeps}
          '';

          meta = with pkgs.lib; {
            description = "A TUI-first CLI utility for burning ISO images to USB drives";
            homepage = "https://github.com/yourusername/ferric";
            license = licenses.mit;
            maintainers = [ simoncarucci@gmail.com ];
            platforms = platforms.linux;
          };
        };

        packages.ferric = self.packages.${system}.default;
      }
    );
}
