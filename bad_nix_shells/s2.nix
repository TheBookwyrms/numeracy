{ pkgs ? import <nixpkgs> {} }:

let
    rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
    cargo2nix = import (builtins.fetchTarball "https://github.com/cargo2nix/cargo2nix/tarball/master") {};
    #pkgs = pkgs { overlays = [ rust_overlay ]; };
    pkgs.overlays = [ rust_overlay ];
    unstable = import <unstable> {};
    rust = pkgs.rust-bin.stable."1.54.0".default.override {
      extensions = [ "rust-src" ];
    };

in pkgs.mkShell {
    buildInputs = [
      pkgs.exiftool
      pkgs.file
      pkgs.pkgconfig
      pkgs.glib
      pkgs.gtk3
      pkgs.wrapGAppsHook
      rust
      cargo2nix.package
      unstable.rust-analyzer
    ];

    shellHook = ''
      if [ -e ~/.nixpkgs/shellhook.sh ]; then . ~/.nixpkgs/shellhook.sh; fi

      export MAGIC_DB=${pkgs.file}/share/misc/magic.mgc;
    '';
}