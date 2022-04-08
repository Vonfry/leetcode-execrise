{ rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "leetcode-cn_2181";
  version = "0.0.1";

  src = ./.;

  cargoSha256 = "";
}
