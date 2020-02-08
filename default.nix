{ }:
let
  pkgs = import <nixpkgs> { };
  sources = import ./nix/sources.nix;
  naersk = pkgs.callPackage sources.naersk { };
  pkg = naersk.buildPackage {
    src = ./.;
    buildInputs = [ pkgs.openssl pkgs.pkg-config ];
  };
in pkg
