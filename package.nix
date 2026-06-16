{ lib
, pkg-config
, rustPlatform
,
}:
rustPlatform.buildRustPackage {
  pname = "mass-binary-versions";
  version = "0.1.0";
  src = lib.cleanSource ./.;
  nativeBuildInputs = [ pkg-config ];
  doCheck = true;
  cargoLock.lockFile = ./Cargo.lock;
  meta = {
    description = "Cache Music Assistant AirPlay helper artifact provenance in SQLite";
    license = lib.licenses.mit;
    mainProgram = "mass-binary-versions";
    platforms = lib.platforms.unix;
  };
}
