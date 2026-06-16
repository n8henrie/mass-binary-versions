{
  description = "Music Assistant AirPlay helper artifact provenance cache";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      eachSystem =
        with nixpkgs.lib;
        f: foldAttrs mergeAttrs { } (map (s: mapAttrs (_: v: { ${s} = v; }) (f s)) systems);
    in
    eachSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = {
          default = self.outputs.packages.${system}.mass-binary-versions;
          mass-binary-versions = pkgs.callPackage ./package.nix { };
          ci-update = pkgs.writeShellApplication {
            name = "ci-update";
            runtimeInputs = [ pkgs.git ];
            text = builtins.readFile ./scripts/ci-updates.sh;
          };
        };

        apps = {
          default = self.outputs.apps.${system}.mass-binary-versions;
          mass-binary-versions = {
            type = "app";
            program = pkgs.lib.getExe self.packages.${system}.mass-binary-versions;
          };
          ci-update = {
            type = "app";
            program = pkgs.lib.getExe self.packages.${system}.ci-update;
          };
        };

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              pkg-config
              rustc
              rustfmt
              sqlite
            ];
          };
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
