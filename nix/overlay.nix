final: prev:

let
  rv32_pkgs = final.pkgsCross.riscv32-embedded;
  rv32_buildPkgs = rv32_pkgs.buildPackages;
in
{
  inherit rv32_pkgs rv32_buildPkgs; # for easier inspection
  libspike = final.callPackage ./pkgs/libspike.nix { };
  libspike-interfaces = final.callPackage ./pkgs/libspike-interfaces.nix { };
}
