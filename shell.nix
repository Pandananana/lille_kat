{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = with pkgs.buildPackages; [ 
      rustup
      cargo 
      pkg-config
    ];

    # Set environment variables for temporary directories
    shellHook = ''
      export TMPDIR=/tmp
    '';
}
