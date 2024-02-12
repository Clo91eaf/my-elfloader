final: prev:

{
  libspike = final.callPackage ./pkgs/libspike.nix { };
  libspike-interfaces = final.callPackage ./pkgs/libspike-interfaces.nix { };
  myRustToolchain = final.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
  my-elfloader = final.callPackage ./pkgs/my-elfloader.nix { };
  t1 = final.callPackage ./t1 { };
}
