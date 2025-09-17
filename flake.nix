{
  description = "cowe.dev backend";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;

      mkRelease =
        pkgs: source:
        let
          manifest = (pkgs.lib.importTOML source/Cargo.toml).package;
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = source/Cargo.lock;
          src = source;
        };
    in
    {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./default.nix { };

        release = mkRelease pkgsFor.${system} pkgsFor.${system}.fetchFromGithub {
          owner = "ScottCowe";
          repo = "cowedev-backend";
          rev = "4a12e0bb6a1861c6ee6aceb557aad2af70cb7a4d";
          sha256 = "sha256-iLUrizRaNMvZMmzJ1dM2HbKelEtzQBNRjgPd0vXCkcM=";
        };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}
