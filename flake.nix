{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {
        config,
        pkgs,
        system,
        lib,
        self',
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        formatter = pkgs.alejandra;

        packages = {
          stable-rust-toolchain = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = ["rust-src" "clippy"];
          };

          nightly-rust-toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rust-src" "clippy"];
            });

          nightly-rustfmt = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rustfmt"];
            });

          nightly-rust-analyzer = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.minimal.override {
              extensions = ["rust-analyzer"];
            });
        };

        devShells.default = pkgs.mkShell {
          packages = [
            config.formatter
            config.packages.nightly-rust-toolchain
            config.packages.nightly-rust-analyzer
            config.packages.nightly-rustfmt
            pkgs.cargo-nextest
            pkgs.fd
            pkgs.hyperfine
            pkgs.just
            pkgs.nodePackages.prettier
            pkgs.taplo
          ];
        };
      };
    };
}
