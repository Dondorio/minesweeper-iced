let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs;
      lib.makeLibraryPath [
        wayland
        libxkbcommon
        fontconfig
      ]}";

    buildInputs = with pkgs; [
      cargo
      gtk3
      openssl
      pkg-config
      rustc
    ];

    ICED_BACKEND = "tiny-skia";
    RUST_BACKTRACE = 1;
    WINEARCH = "win64";
  }
