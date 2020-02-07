let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  sources = import ./nix/sources.nix;
  naersk = pkgs.callPackage sources.naersk { };
in naersk.buildPackage ./.
