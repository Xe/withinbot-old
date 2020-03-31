{ sources ? import ../nix/sources.nix
, zig ? import ../nix/zig.nix { inherit sources; }
, pkgs ? import sources.nixpkgs { } }:

with pkgs;
stdenv.mkDerivation {
  name = "libstonks";
  version = "latest";
  src = ./.;

  buildInputs = [ clang zig ];
  phases = "buildPhase installPhase";

  buildPhase = ''
    cp -rf $src/src/* .
    export XDG_CACHE_HOME=$(pwd)
    zig c++ -g -c -Wall -Werror -fPIC stonks.cpp
    zig c++ -shared -o libstonks.so stonks.o
  '';

  installPhase = ''
    mkdir -p $out/lib $out/include
    cp -vrf libstonks.so $out/lib/libstonks.so
    cp -vrf libstonks.h $out/include/libstonks.h
  '';
}
