{ makeRustPlatform
, myRustToolchain

  # dev deps
, libspike-interfaces
}:
let
  myRustPlatform = makeRustPlatform {
    cargo = myRustToolchain;
    rustc = myRustToolchain;
  };

  self = myRustPlatform.buildRustPackage {
    pname = "my-elfloader";
    version = "0.1.0";

    src = ../../.;

    # Link time dependencies
    buildInputs = [ libspike-interfaces ];

    cargoLock = {
      lockFile = ../../Cargo.lock;
    };
  };
in
self