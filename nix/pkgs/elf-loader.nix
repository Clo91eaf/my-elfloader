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

  self = myRustPlatform.buildRustPackage
    {
      pname = "elf-loader";
      version = "0.1.0";

      src = ../../.;

      # Build time & Runtime dependencies
      nativeBuildInputs = [ pkg-config llvmPackages_16.bintools ];
      # Link time dependencies
      buildInputs = [ libspike-interfaces ];

      cargoLock = {
        lockFile = ../Cargo.lock;
      };
    };
in
self