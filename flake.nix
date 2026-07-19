{
  description = "Dynamic Discord Rich Presence based on active Hyprland / Niri windows";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        packages = {
          dynamic-drpc-wayland = pkgs.callPackage ./nix/package.nix { };
          default = self.packages.${system}.dynamic-drpc-wayland;
        };

        formatter = pkgs.nixfmt;
      }
    )
    // {
      homeManagerModules.default = ./nix/home-manager.nix;
    };
}
