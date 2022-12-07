{
  description = "wash - WASMCloud Shell";

  inputs = {
    flakeutils.url       = "github:numtide/flake-utils";
    nixpkgs.url          = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url           = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flakeutils, naersk }:
    flakeutils.lib.eachDefaultSystem (system:
      let
        pkgs        = (import nixpkgs) { inherit system; };
        naersk'     = pkgs.callPackage naersk {};
	cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));
	name        = "wash";

	supportedSystems = [ "x86_64-darwin" "aarch64-linux" ];
        forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);

      in rec {
        packages.${name} = naersk'.buildPackage {
          pname = "${name}";
          root = ./.;

          # Workaround for lack of a naersk option to select --bin target.
          singleStep = true;
          cargoBuildOptions = (opts: opts ++ ["--bin=wash"]);

	  nativeBuildInputs = with pkgs; [
            pkgconfig
            clang
            grpc-tools
	  ];

          buildInputs = with pkgs; [
            llvmPackages.libclang
          ];

          propagatedBuildInputs = with pkgs; [
            openssl
          ];

          runtimeDependencies = with pkgs; [
            openssl
          ];


          # Allow build step to find libclang.so path.
          LD_LIBRARY_PATH = "${pkgs.llvmPackages.libclang}/lib/";
        };
        

	packages.default = packages.${name};
	
	apps.${name} = flakeutils.lib.mkApp {
          drv = packages.${name};
        };
      });
}
