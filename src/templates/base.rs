use crate::templates::{Command, File, Folder, Template, TemplateConfig};

#[derive(Debug)]
pub struct Base;

impl Template for Base
{
    fn default() -> TemplateConfig
    {
        TemplateConfig {
            dir_structure: vec![
                Folder {
                    name: String::from("src"),
                    sub_dirs: vec![Folder {
                        name: String::from("utils"),
                        sub_dirs: vec![],
                    }],
                },
                Folder {
                    name: String::from("bin"),
                    sub_dirs: vec![],
                },
            ],

            files: vec![File {
                path: String::from("flake.nix"),
                content: String::from(FLAKE_NIX),
            }],

            build: vec![
                Command {
                    program: String::from("git"),
                    args: vec![String::from("add"), String::from("-A")],
                },
                Command {
                    program: String::from("nix"),
                    args: vec![String::from("develop")],
                },
            ],

            run: vec![Command {
                program: String::from("kitty"),
                args: vec![String::from("--detach")],
            }],
        }
    }

    fn template_path() -> &'static str
    {
        "templates/base.json"
    }
}

const FLAKE_NIX: &str = r#"{
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

      buildInputs = with pkgs; [
        glib
        openssl
      ];
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
        ${pkgs.onefetch}/bin/onefetch

        alias run="cargo run"
        alias add="cargo add"
        alias build="nix build"
      '';
    };
  };
}
"#;
