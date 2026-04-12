{pkgs}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    (rust-bin.nightly."2026-02-01".default.override {
      extensions = ["rust-src" "rust-analyzer" "clippy" "rustfmt"];
    })
    glib
    just
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
}
