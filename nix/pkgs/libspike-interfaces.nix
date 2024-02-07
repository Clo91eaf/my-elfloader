{ stdenv, lib, cmake, libspike }:

stdenv.mkDerivation {
  name = "libspike-interfaces";

  src = (with lib.fileset; toSource {
      root = ./../..;
      fileset = unions [
        ./../../csrc
      ];
    }).outPath;

  nativeBuildInputs = [
    cmake
  ];

  buildInputs = [
    libspike
  ];
}
