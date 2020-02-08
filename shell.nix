let
  pkgs = import <nixpkgs> { };
  sources = import ./nix/sources.nix;
in pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rls
    pkgs.cargo
    pkgs.openssl
    pkgs.pkg-config
    sources.niv
  ];
}
