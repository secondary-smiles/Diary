{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let 
    system = "aarch64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = 
    pkgs.mkShell {
      buildInputs = with pkgs; [
        rustc
        rustfmt
        cargo
        rust-analyzer
        clippy
        pkg-config
      ];
      shellHook = ''
        exec zsh
      '';
    };
  };
}
