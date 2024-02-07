{ stdenv, cmake, libspike }:

stdenv.mkDerivation {
  name = "libspike-interfaces";

  src = ../../csrc;

  nativeBuildInputs = [
    cmake
  ];

  buildInputs = [
    libspike
  ];
}
