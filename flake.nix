{
  description = "Rust latest workspace";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    # Add dependencies that are only needed for development
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        {
          devShells. default = let p = pkgs; in
            pkgs.mkShell {
              buildInputs =
                [
                  p.rust-bin.stable.latest.default
                  p.rust-analyzer
                  p.cmake
                  p.freetype
                  p.expat
                  p.openssl
                  p.pkgconfig
                  p.python3
                ];
            };
        });
}

