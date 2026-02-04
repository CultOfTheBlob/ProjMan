{
  description = "Rust development enviroment.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    naersk,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";

    overlays = [(import rust-overlay)];

    pkgs = import nixpkgs {
      inherit system overlays;
    };
    naerskLib = pkgs.callPackage naersk {};
  in {
    packages."x86_64-linux".default = naerskLib.buildPackage {
      src = ./.;

      buildInputs = [pkgs.glib];
      nativeBuildInputs = [pkgs.pkg-config];
    };

    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        (
          rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "rustfmt"
              ];
            })
        )

        cargo
        clippy
        glib
      ];

      nativeBuildInputs = [pkgs.pkg-config];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

      shellHook = ''
        ${pkgs.onefetch}/bin/onefetch

        alias run="cargo run"
        alias add="cargo run"
        alias build="nix build"
      '';
    };
  };
}
