{ }:
let
  pkgs = import <nixpkgs> { };
  sources = import ./nix/sources.nix;
  naersk = pkgs.callPackage sources.naersk { };
  pkg = naersk.buildPackage {
    src = builtins.filterSource
      (path: type: type != "directory" || builtins.baseNameOf path != "target")
      ./.;
    buildInputs = [ pkgs.openssl pkgs.pkg-config ];
  };
in pkg
