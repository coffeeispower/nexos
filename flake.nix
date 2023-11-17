{
  description = "Tinyx Development Environment";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          OVMF_PATH = "${pkgs.OVMF.fd}/FV/OVMF.fd";
          AAVMF_PATH = "${pkgs.OVMF.fd}/FV/AAVMF.fd";
          buildInputs = [
            qemu_full
            xorriso
            gnumake
            git
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          ];
        };
      }
    );
}
