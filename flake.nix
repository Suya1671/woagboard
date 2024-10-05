{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { nixpkgs, ... }: let
    systems = [ "x86_64-linux" ];
    forAllSystems = nixpkgs.lib.genAttrs systems;
  in {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };

      pyPkgs = pythonPackages: with pythonPackages; [
        intelhex
      ];
    in {
      default = pkgs.mkShell {
        packages = [
          (pkgs.python3.withPackages pyPkgs)
          pkgs.cmake
          pkgs.ninja
          pkgs.adafruit-nrfutil
          pkgs.gcc-arm-embedded
        ];
      };
    });
  };
}
