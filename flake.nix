{
  description = "Rust development environment.";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};
        naerskLib = pkgs.callPackage naersk {};
      in {
        packages.default = naerskLib.buildPackage {
          src = ./.;
          buildInputs = with pkgs; [glib openssl];
          nativeBuildInputs = with pkgs; [pkg-config makeWrapper];
          OPENSSL_NO_VENDOR = 1;

          postInstall = ''
            wrapProgram $out/bin/projman \
              --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath (with pkgs; [
              wayland
              libxkbcommon
              libGL
              mesa
              vulkan-loader
            ])}
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.selectLatestNightlyWith (toolchain:
              toolchain.default.override {
                extensions = ["rust-src" "rust-analyzer" "rustfmt"];
              }))
            cargo
            clippy
            glib
            openssl.dev
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.libGL
            pkgs.mesa
            pkgs.vulkan-loader
          ];
          nativeBuildInputs = [pkgs.pkg-config];
          env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          shellHook = ''
            alias run="cargo run"
            alias add="cargo add"
            alias build="nix build"
          '';
        };
      }
    );
}
