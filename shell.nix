let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  niv = (import sources.niv { }).niv;
  zig = import ./nix/zig.nix { inherit sources; };
  stonks = import ./turnips { inherit sources zig; };
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
    stonks

    # zig
    zig

    # tooling
    niv
  ];

  DATABASE_URL="target/withinbot.db";
}
