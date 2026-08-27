{ pkgs ? import <nixpkgs> {} }:
let
  # rust-overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  # #cargo2nix = import (builtins.fetchTarball "https://github.com/cargo2nix/cargo2nix/tarball/master") {};
  #             #nixpkgs.overlays = [ rust-overlay.overlays.default ];
  # #pkgs = pkgs { overlays = [ rust_overlay ]; };
  # pkgs.overlays = [ rust-overlay.overlays.default ];

  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = (import <nixpkgs> {
    overlays = [ rust-overlay ];
  });
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
in pkgs.mkShell {

  nativeBuildInputs = [
    pkgs.rust-analyzer
  ];

  packages = [
    (pkgs.python312.withPackages(p: with p; [
      numpy
    ]))
    rust
    #pkgs.cargo
    #pkgs.rustc
  ];


    #shellHook = ''
    #    export TEMPDIR="$(mktemp)"
    #'';
    #shellHook = ''
    #    export TEMPDIR="$(mktemp -d /)"
    #    export TEMPDIR="$(mktemp -d /tmp/nix-shell-XXXXXX)"
    #    export TEMPDIR="$(mktemp -d /tmp/nix-shell-XXXXXX-XXXXXX/rustdoctestNfnMJn)"
    #'';

    #RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";



}