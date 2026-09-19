{ pkgs ? import <nixpkgs> {} }:
let

  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = (import <nixpkgs> {
    overlays = [ rust-overlay ];
  });
    rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      extensions = [ "rust-src" ];
    });
    
in pkgs.mkShell {

  nativeBuildInputs = [
    pkgs.rust-analyzer
  ];

  packages = [
    rust
  ];

}