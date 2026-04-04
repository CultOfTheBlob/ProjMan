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

          postInstall = let
            desktopEntry = pkgs.lib.generators.toINI {} {
              "Desktop Entry" = {
                Name = "ProjMan";
                Exec = "${placeholder "out"}/bin/projman";
                Icon = "projman";
                Type = "Application";
                Categories = "Utility;";
              };
            };
          in ''
              wrapProgram $out/bin/projman \
              --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath (with pkgs; [
              wayland
              libxkbcommon
              libGL
              mesa
              vulkan-loader
            ])}

            install -Dm644 ${./assets/icon.svg} $out/share/icons/hicolor/scalable/apps/projman.svg

            mkdir -p $out/share/applications
            echo "${desktopEntry}" > $out/share/applications/projman.desktop
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
    )
    // {
      homeManagerModules.default = {
        config,
        lib,
        pkgs,
        ...
      }: let
        cfg = config.programs.projman;
        package = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
        tomlFormat = pkgs.formats.toml {};
      in {
        options.programs.projman = {
          enable = lib.mkEnableOption "projman";

          settings = {
            general = {
              projects_dir = lib.mkOption {
                type = lib.types.str;
                default = "${config.home.homeDirectory}/Projects/";
                description = "Path to the projects directory.";
              };
              delete_project_folder = lib.mkOption {
                type = lib.types.bool;
                default = false;
                description = "Whether to delete the project folder when removing a project.";
              };
            };
            theme = {
              theme = lib.mkOption {
                type = lib.types.str;
                default = "Dark";
                description = "Theme to use.";
              };
            };
          };

          templates = lib.mkOption {
            type = lib.types.attrsOf (lib.types.submodule {
              options = {
                dir_structure = lib.mkOption {
                  type = lib.types.listOf (lib.types.submodule {
                    options = {
                      name = lib.mkOption {type = lib.types.str;};
                      sub_dirs = lib.mkOption {
                        type = lib.types.listOf lib.types.attrs;
                        default = [];
                      };
                    };
                  });
                  default = [];
                };

                files = lib.mkOption {
                  type = lib.types.listOf (lib.types.submodule {
                    options = {
                      path = lib.mkOption {type = lib.types.str;};
                      content = lib.mkOption {
                        type = lib.types.str;
                        default = "";
                      };
                      tracked = lib.mkOption {
                        type = lib.types.bool;
                        default = true;
                      };
                    };
                  });
                  default = [];
                };

                build = lib.mkOption {
                  type = lib.types.listOf (lib.types.submodule {
                    options = {
                      program = lib.mkOption {type = lib.types.str;};
                      args = lib.mkOption {
                        type = lib.types.listOf lib.types.str;
                        default = [];
                      };
                    };
                  });
                  default = [];
                };

                run = lib.mkOption {
                  type = lib.types.listOf (lib.types.submodule {
                    options = {
                      program = lib.mkOption {type = lib.types.str;};
                      args = lib.mkOption {
                        type = lib.types.listOf lib.types.str;
                        default = [];
                      };
                    };
                  });
                  default = [];
                };

                included_paths = lib.mkOption {
                  type = lib.types.listOf lib.types.str;
                  default = [];
                };

                excluded_paths = lib.mkOption {
                  type = lib.types.listOf lib.types.str;
                  default = [];
                };
              };
            });
            default = {};
            description = "Definitons for templates. Each attribute maps to a json file of the same name.";
          };

          icons = lib.mkOption {
            type = lib.types.attrsOf lib.types.path;
            default = {};
            description = "Icons for templates. Attribute name should match the template name.";
          };

          projects = lib.mkOption {
            type = lib.types.listOf (lib.types.submodule {
              options = {
                name = lib.mkOption {type = lib.types.str;};
                path = lib.mkOption {type = lib.types.str;};
                template_name = lib.mkOption {
                  type = lib.types.str;
                  default = "";
                };
                repo = lib.mkOption {
                  type = lib.types.str;
                  default = "";
                };
                license = lib.mkOption {
                  type = lib.types.str;
                  default = "";
                };
              };
            });
            default = [];
            description = "List of projects in the projects.json file.";
          };
        };

        config = lib.mkIf cfg.enable {
          home.packages = [package];
          xdg.configFile =
            {
              projman = {
                target = "projman/config.toml";
                source = tomlFormat.generate "config.toml" {
                  general = {
                    inherit (cfg.settings.general) projects_dir delete_project_folder;
                  };
                  theme = {
                    inherit (cfg.settings.theme) theme;
                  };
                };
              };
            }
            // (lib.mapAttrs' (
                name: template:
                  lib.nameValuePair "projman-template-${name}" {
                    target = "projman/templates/${name}.json";
                    text = builtins.toJSON template;
                  }
              )
              cfg.templates)
            // (lib.mapAttrs' (
                name: icon: let
                  filename = baseNameOf (toString icon);
                  ext = builtins.head (builtins.match ".*(\\..*)" filename);
                in
                  lib.nameValuePair "projman-icon-${name}" {
                    target = "projman/icons/${name}${ext}";
                    source = icon;
                  }
              )
              cfg.icons);

          home.activation.projmanProjects = lib.hm.dag.entryAfter ["writeBoundary"] ''
            if [ ! -f "$HOME/.config/projman/projects.json" ]; then
              echo '${builtins.toJSON (map (p: p // {exists = true;}) cfg.projects)}' > "$HOME/.config/projman/projects.json"
            fi
          '';
        };
      };
    };
}
