{
  description = "numeracy";

  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };
  };

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
    pkgs = (import nixpkgs {
      overlays = [ rust-overlay ];
    });
    rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-src" ];
    });
  in {
    devShells.${system}.default = pkgs.mkShell {

      nativeBuildInputs = [
        pkgs.rust-analyzer
      ];

      packages = [
        rust
      ];

    };
  };
}