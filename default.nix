{
  rustPlatform,
  clippy,
  rustfmt,
  cmake,
  pkg-config,
  ...
}:

rustPlatform.buildRustPackage {

  pname = "waveshare-st7789vm";
  version = "0.1.0";

  src = ./.;

  nativeBuildInputs = [
    cmake
    pkg-config
    rustfmt
    clippy
  ];

  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
