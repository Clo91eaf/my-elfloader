{
  description = "Elf loader";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlay = import ./nix/overlay.nix;
        overlays = [ (import rust-overlay) overlay ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        legacyPackages = pkgs;
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rs-toolchain
            pkgs.rust-analyzer-unwrapped
          ];

          RUST_SRC_PATH = "${rs-toolchain}/lib/rustlib/src/rust/library";
        };
      }
    )
    // { inherit inputs; overlays.default = overlay; };
}
