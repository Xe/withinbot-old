{ sources ? import ./sources.nix
, xepkgs ? import sources.xepkgs { pkgs = (import sources.nixpkgs { }); } }:

xepkgs.zig "0.5.0+9e019ed26" {
  mac = "ab18f33879ef1fd5b485b82f264a735419c7253930ea9f460e29ae828117e5db";
  linux = "6bc625b3fd171bd61854bdcc102054aa6f3b57bd653149cba99a841e289ff7d1";
}
