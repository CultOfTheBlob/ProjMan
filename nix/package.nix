{
  self,
  pkgs,
  naerskLib,
}:
naerskLib.buildPackage {
  src = self;
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

    install -Dm644 ${../assets/icon.svg} $out/share/icons/hicolor/scalable/apps/projman.svg

    mkdir -p $out/share/applications
    echo "${desktopEntry}" > $out/share/applications/projman.desktop
  '';
}
