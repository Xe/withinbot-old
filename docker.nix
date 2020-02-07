{ system ? builtins.currentSystem }:

let
  pkgs = import <nixpkgs> { inherit system; };

  callPackage = pkgs.lib.callPackageWith pkgs;

  withinbot = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildImage {
      name = "xena/withinbot";
      tag = pkg.version;

      contents = [ pkg ];

      config = {
        Cmd = [ "/bin/withinbot" ];
        WorkingDir = "/";
      };
    };

in dockerImage withinbot
