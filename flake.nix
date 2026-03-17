{
  description = "NILOO: niloo is local only openid";

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          f {
            inherit system;
            pkgs = import nixpkgs { inherit system; };
          }
        );
    in
    {
      packages = forAllSystems (
        { pkgs, ... }:
        let
          niloo = pkgs.rustPlatform.buildRustPackage {
            pname = "niloo";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        in
        {
          inherit niloo;
          default = niloo;
        }
      );

      apps = forAllSystems (
        { system, ... }:
        let
          program = "${self.packages.${system}.niloo}/bin/niloo";
        in
        {
          niloo = {
            type = "app";
            inherit program;
          };
          default = {
            type = "app";
            inherit program;
          };
        }
      );

      devShells = forAllSystems (
        { pkgs, ... }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              clippy
              git
              nixfmt-rfc-style
              rust-analyzer
              rustc
              rustfmt
            ];
          };
        }
      );
    };
}
