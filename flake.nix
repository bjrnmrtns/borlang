{
  description = "borlang";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ self, nixpkgs }:
    {
      packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.rustPlatform.buildRustPackage {
        pname = "borlang";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };

      devShell.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.mkShell {
        name = "borlang-dev-shell";
        buildInputs = with nixpkgs.legacyPackages.x86_64-linux; [
          cargo
          cargo-watch
          rustc
          rustfmt
          rust-analyzer
          clippy
          cmake
          rust-docs
        ];
        shellHook = ''
          export RUST_LOG=debug
          echo "borlang dev shell!"
        '';
      };
    };
}
