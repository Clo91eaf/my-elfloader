{
  description = "My elf loader";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    t1.url = "github:chipsalliance/t1";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, t1 }@inputs:
    let
      overlay = import ./nix/overlay.nix;
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) t1.overlays.default overlay ];
        pkgs = import nixpkgs { inherit system overlays; };
        rs-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        legacyPackages = pkgs;
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Including latest cargo,clippy,cargo-fmt
            rs-toolchain
            # rust-analyzer comes from nixpkgs toolchain, I want the unwrapped version
            pkgs.rust-analyzer-unwrapped
          ];

          # To make rust-analyzer work correctly (The path prefix issue)
          RUST_SRC_PATH = "${rs-toolchain}/lib/rustlib/src/rust/library";
        };
      }
    )
  // { inherit inputs; overlays.default = overlay; };
}
