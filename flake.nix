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
    in
    {
      packages = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./default.nix { };

        release =
          let
            manifest = (pkgsFor.${system}.lib.importTOML ./Cargo.toml).package;
          in
          pkgsFor.${system}.rustPlatform.buildRustPackage {
            pname = manifest.name;
            version = manifest.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgsFor.${system}.fetchFromGitHub {
              owner = "ScottCowe";
              repo = "cowedev-backend";
              rev = "cea25a8bddabbe19793e6c3cc2fa446101e204b3";
              sha256 = "sha256-n2tbKIS2Y8xVPWkpK24WnjSbz0O5adVHiwr0/JA0YmQ=";
            };
          };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
    };
}
