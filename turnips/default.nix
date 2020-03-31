{ sources ? import ../nix/sources.nix
, zig ? import ../nix/zig.nix { inherit sources; }
, pkgs ? import sources.nixpkgs { } }:

with pkgs;
stdenv.mkDerivation {
  name = "libstonks";
  version = "latest";
  src = ./.;

  buildInputs = [ zig ];
  phases = "buildPhase installPhase";

  buildPhase = ''
    cp -rf $src/src/* .
    export XDG_CACHE_HOME=$(pwd)
    ${zig}/bin/zig c++ -c -Wall -Werror -fpic TurnipPrices.cpp
    ${zig}/bin/zig c++ -shared -o libstonks.so TurnipPrices.o
  '';

  installPhase = ''
    mkdir -p $out/lib $out/include
    cp -vrf libstonks.so $out/lib/libstonks.so
    cp -vrf libstonks.h $out/include/libstonks.h
  '';
}
