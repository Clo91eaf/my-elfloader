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
      outputHashes = {
        "fst-native-0.6.3" = "sha256-18g/euQQjS8euF+za7za9/0/esGkfZfioB1rlAt59S4=";
      };
    };
  };
in
self