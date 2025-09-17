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
          rev = "f282f260b342d5ff8432567af0268cf8a1e01d4e";
          sha256 = "sha256-Ui4kDJNedA2xVT94EQTTHAnMvMrP9UjLqR9t1agtsZE=";
        };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}
