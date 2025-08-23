{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];

  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
    cargo-watch
    postgresql
  ];

  shellHook = ''
    ./postgres.sh start

    trap "./postgres.sh stop" EXIT
  '';
}
