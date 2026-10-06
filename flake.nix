{
  description = "Bektor/Vec-Utils-Py a rust and python vector library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
    ];

    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

    pkgsFor = system:
      import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
  in {
    packages = forAllSystems (system: let
      pkgs = pkgsFor system;
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
    in {
      default = pkgs.python3Packages.buildPythonPackage {
        pname = "vec-utils-py";
        version = "0.3.2";
        src = ./.;
        pyproject = true;

        cargoDeps = pkgs.rustPlatform.importCargoLock {
          lockFile = ./Cargo.lock;
        };

        preBuild = ''
          cd vec-utils-py
        '';

        build-system = [
          pkgs.maturin
        ];

        nativeBuildInputs = [
          rustToolchain
          pkgs.maturin
          rustPlatform.cargoSetupHook
          rustPlatform.maturinBuildHook
        ];

        propagatedBuildInputs = [];

        pythonImportsCheck = ["vec_utils_py"];
      };

      crate = rustPlatform.buildRustPackage {
        pname = "bektor";
        version = "0.3.2";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        cargoBuildFlags = ["-p" "bektor"];
        doCheck = false;

        dontCargoInstall = true;
        postInstall = ''
          mkdir -p $out
        '';

        nativeBuildInputs = [rustToolchain];
      };
    });

    devShells = forAllSystems (system: let
      pkgs = pkgsFor system;
      rustToolchain = pkgs.rust-bin.nightly.latest.default;
      pythonEnv = pkgs.python3.withPackages (ps:
        with ps; [
          pip
          pytest
        ]);
    in {
      default = pkgs.mkShell {
        packages = [
          rustToolchain
          pythonEnv
          pkgs.maturin
        ];

        shellHook = ''
          export CARGO_BUILD_TARGET_DIR="$PWD/target"
        '';
      };
    });
  };
}
