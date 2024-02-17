{ stdenv, cmake, libspike, fmt }:

stdenv.mkDerivation {
  name = "libspike-interfaces";

  src = ../../csrc;

  nativeBuildInputs = [
    cmake
  ];

  buildInputs = [
    libspike
    fmt
  ];
}
