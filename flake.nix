{
  description = "Rust development environment.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

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
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      naerskLib = pkgs.callPackage naersk {};
    in {
      packages.default = naerskLib.buildPackage {
        src = self;

        buildInputs = with pkgs; [
          glib
          libxcb
          libxkbcommon
          fontconfig
          vulkan-loader
          pango
          atk
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          makeWrapper
        ];

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
            libxcb
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
          (rust-bin.nightly.latest.default.override {
            extensions = ["rust-src" "rust-analyzer" "clippy" "rustfmt"];
          })
          glib
          just

          libxcb
          libxkbcommon
          fontconfig
          pango
          atk
          openssl
        ];

        LD_LIBRARY_PATH = with pkgs;
          pkgs.lib.makeLibraryPath [
            libxcb
            wayland
            libxkbcommon
            libGL
            mesa
            vulkan-loader
          ];

        nativeBuildInputs = [pkgs.pkg-config];
      };
    })
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
        yamlFormat = pkgs.formats.yaml {};

        rgbaType = lib.types.strMatching "^#[0-9a-fA-F]{8}$";
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
                type = lib.types.enum [
                  "Dark"
                  "Light"
                  "Nord"
                  "NordLight"
                  "GruvboxDark"
                  "GruvboxLight"
                  "TokyoNightDark"
                  "TokyoNightLight"
                  "CatppuccinFrappe"
                  "CatppuccinLatte"
                  "CatppuccinMacchiato"
                  "CatppuccinMocha"
                  "Custom"
                ];
                default = "Dark";
                description = "Theme to use.";
              };

              custom = lib.mkOption {
                type = lib.types.nullOr (
                  lib.types.submodule {
                    options = {
                      background = lib.mkOption {type = rgbaType;};
                      background_weak = lib.mkOption {type = rgbaType;};
                      surface = lib.mkOption {type = rgbaType;};
                      surface_strong = lib.mkOption {type = rgbaType;};
                      border = lib.mkOption {type = rgbaType;};
                      text_disabled = lib.mkOption {type = rgbaType;};
                      text_muted = lib.mkOption {type = rgbaType;};
                      text = lib.mkOption {type = rgbaType;};
                      text_strong = lib.mkOption {type = rgbaType;};
                      error = lib.mkOption {type = rgbaType;};
                      warning = lib.mkOption {type = rgbaType;};
                      info = lib.mkOption {type = rgbaType;};
                      success = lib.mkOption {type = rgbaType;};
                      accent = lib.mkOption {type = rgbaType;};
                      accent_alt = lib.mkOption {type = rgbaType;};
                      accent_muted = lib.mkOption {type = rgbaType;};
                      special = lib.mkOption {type = rgbaType;};
                    };
                  }
                );
                default = null;
                description = "Custom theme configuration when settings.theme.theme is set to 'Custom'.";
              };
            };
          };

          templates = lib.mkOption {
            type = lib.types.attrsOf (lib.types.submodule {
              options = {
                icon = lib.mkOption {
                  type = lib.types.path;
                  description = "Path to the icon file for this template.";
                };

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
                      contents = lib.mkOption {
                        type = lib.types.str;
                        default = "";
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
            description = "Definitions for templates.";
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
            description = "List of projects in the projects.yaml file.";
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
                    theme =
                      if cfg.settings.theme.theme == "Custom"
                      then {
                        Custom = cfg.settings.theme.custom;
                      }
                      else cfg.settings.theme.theme;
                  };
                };
              };
            }
            // (lib.mapAttrs' (
                name: template:
                  lib.nameValuePair "projman-template-${name}" {
                    target = "projman/templates/${name}/template.yaml";
                    source = yamlFormat.generate "template.yaml" (removeAttrs template ["icon"]);
                  }
              )
              cfg.templates)
            // (lib.mapAttrs' (
                name: template:
                  lib.nameValuePair "projman-icon-${name}" {
                    target = "projman/templates/${name}/icon.svg";
                    source = template.icon;
                  }
              )
              cfg.templates);

          home.activation.projmanProjects = let
            projectsYaml = yamlFormat.generate "projects.yaml" (map (p: p // {exists = true;}) cfg.projects);
          in
            lib.hm.dag.entryAfter ["writeBoundary"] ''
              if [ ! -f "$HOME/.config/projman/projects.yaml" ]; then
                cp "${projectsYaml}" "$HOME/.config/projman/projects.yaml"
              fi
            '';
        };
      };
    };
}
