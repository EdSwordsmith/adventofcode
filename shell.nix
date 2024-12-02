{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
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
    julia_110-bin
    # Go
    go
    # OCaml
    dune_3
    ocaml
    ocamlPackages.odoc
    ocamlPackages.utop
    ocamlPackages.findlib
    ocamlPackages.angstrom
  ];
}
