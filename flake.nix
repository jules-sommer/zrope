{
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    gitignore,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        inherit (pkgs) lib;
        inherit (gitignore.lib) gitignoreSource;
        pkgs = import nixpkgs {
          inherit system;
          overlays = import ./overlays.nix inputs;
        };

        nativeBuildInputs = with pkgs; [
          zig
          zls
          neovim
          pkg-config
        ];

        # libraries the program needs go here
        buildInputs = [
        ];
      in {
        packages.default = pkgs.stdenv.mkDerivation {
          name = "zig-binary";
          version = "0.1.0";
          src = gitignoreSource ./src;

          inherit
            buildInputs
            nativeBuildInputs
            ;
        };

        devShells.default = pkgs.mkShell {
          inherit
            buildInputs
            nativeBuildInputs
            ;
        };
      }
    );

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    zig = {
      url = "github:silversquirl/zig-flake/compat";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    zls = {
      url = "github:zigtools/zls";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        zig-overlay.follows = "zig";
      };
    };

    gitignore.url = "github:hercules-ci/gitignore.nix";
    flake-utils.url = "github:numtide/flake-utils";
    neovim.url = "git+https://git.nixfox.ca/Jules/neovim-flake.git";
  };
}
