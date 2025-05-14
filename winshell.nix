let
  pkgs = import <nixpkgs> {
    localSystem = "x86_64-linux";
    crossSystem.config = "x86_64-w64-mingw32"; # Note the `config` part!
  };

  rust = pkgs.buildPackages.rust-bin.stable.latest.default.override {
    extensions = ["rust-src" "rust-analysis"];
    targets = ["x86_64-pc-windows-gnu"];
  };
in
  pkgs.mkShell {
    nativeBuildInputs = [
      rust
    ];

    buildInputs = with pkgs; [
      windows.mingw_w64_pthreads
      windows.pthreads
    ];
  }
