{
  inputs = {
    zig.url = "github:mitchellh/zig-overlay";
    nixpkgs.url = "github:nixos/nixpkgs";
    zls.url = "github:zigtools/zls";
    flake-utils.url = "github:numtide/flake-utils";

  };

  outputs =
    {
      self,
      zig,
      zls,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            zig.outputs.overlays.default
            (_: prev: {
              inherit (zls.outputs.packages.${prev.system}) zls;
            })
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            zig.outputs.packages.${system}.master
            zls.outputs.packages.${system}.zls
          ];

          shellHook = ''
            fish
          '';
        };
      }
    );
}
