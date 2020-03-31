let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  niv = (import sources.niv { }).niv;
  zig = import ./nix/zig.nix { inherit sources; };
in
with pkgs;

pkgs.mkShell {
  nativeBuildInputs = [ removeReferencesTo ];
  buildInputs = [
    # rust dependencies
    rustc
    rustfmt
    cargo
    rls
    rust-bindgen

    # native dependencies
    openssl
    pkg-config
    sqlite

    # zig
    zig

    # tooling
    niv
  ];

  DATABASE_URL="target/withinbot.db";
}
