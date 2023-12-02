{
  inputs = {
    nixpkgs.url = "flake:nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Rust
          cargo
          rustc
          rustfmt
          clippy
          # C
          gnumake
          clang
          # Python
          python3
          # Julia
          julia-bin
          # OCaml
          dune_3
          ocaml
          ocamlPackages.odoc
          ocamlPackages.utop
          ocamlPackages.findlib
          ocamlPackages.angstrom
        ];
      };
    });
}
