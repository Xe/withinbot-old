let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  niv = (import sources.niv { }).niv;
in
with pkgs;

pkgs.mkShell {
  buildInputs = [
    # rust dependencies
    rustc
    rustfmt
    cargo
    rls

    # native dependencies
    openssl
    pkg-config

    # tooling
    niv
  ];
}
