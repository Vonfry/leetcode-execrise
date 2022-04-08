{ stdenv, rustc }:

stdenv.mkDerivation {
  pname = "leetcode-cn_2181";
  version = "0.0.1";

  src = ./.;

  nativeBuildInputs = [ rustc ];

  buildPhase = ''
    mkdir -p $out/bin
    rustc ./solution.rs -out-dir $out/bin
  '';
}
