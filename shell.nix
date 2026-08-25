{
  pkgs ? import <nixpkgs> { },
}:
pkgs.callPackage (
  {
    mkShell,
    cargo,
    rustc,
    rustfmt,
    rustPlatform,
    pkg-config,
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      cargo
      rustc
      rustfmt
      rustPlatform.bindgenHook
      # optional: add pkg-config support
      pkg-config
    ];
    buildInputs = [
      # add desired native packages
      # ...
    ];
    RUST_SRC_PATH = rustPlatform.rustLibSrc;
    # ...
  }
) { }