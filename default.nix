{ }:
let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  naersk = pkgs.callPackage sources.naersk { };
  pkg = naersk.buildPackage {
    src = builtins.filterSource
      (path: type: type != "directory" || builtins.baseNameOf path != "target")
      ./.;
    buildInputs = [ pkgs.openssl pkgs.pkg-config ];
    doCheck = false;
  };
in pkg
