final: prev:

{
  libspike = final.callPackage ./pkgs/libspike.nix { };
  libspike-interfaces = final.callPackage ./pkgs/libspike-interfaces.nix { };
  myRustToolchain = final.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
  elf-loader = final.callPackage ./pkgs/elf-loader.nix { };
}
