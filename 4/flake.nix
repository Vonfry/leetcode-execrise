{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-21.11";
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      defaultPackage.${system} = pkgs.callPackage ./. { };
      devShell.${system} = with pkgs; mkShell {
        inputsFrom = [ defaultPackage.${system} ];
        buildInputs = [ rust-analyzer ];
      };
    };
}
