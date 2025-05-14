{
  description = "A dev flake";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${with pkgs;
        lib.makeLibraryPath [
          wayland
          libxkbcommon
          fontconfig
        ]}";

      buildInputs = with pkgs; [
        # docker
        podman
        cargo-cross
        cargo
        gtk3
        openssl
        pkg-config
        rustup
        # rustc
      ];

      # ICED_BACKEND = "tiny-skia";
      CROSS_ROOTLESS_CONTAINER_ENGINE = 1;
      CROSS_CONTAINER_ENGINE = "podman";
      RUST_BACKTRACE = 1;
      WINEARCH = "win64";
    };
  };
}
