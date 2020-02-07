let
  moz_overlay = import (builtins.fetchTarball
  "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.latest.rustChannels.nightly.rust
    pkgs.openssl
    pkgs.pkg-config
  ];
}
