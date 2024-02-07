{ stdenv
, lib

  # emulator deps
, cmake

, libspike
}:

stdenv.mkDerivation {
  name = "t1-libspike";

  src = ../../src;

  nativeBuildInputs = [
    cmake
  ];

  buildInputs = [
    libspike
  ];

  meta = {
    mainProgram = "emulator";
    description = "IP emulator for config ";
  };
}
