{
  description = "wash - WASMCloud Shell";

  inputs = {
    flakeutils.url       = "github:numtide/flake-utils";
    nixpkgs.url          = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url           = "github:nix-community/naersk";
    # pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = { self, nixpkgs, flakeutils, naersk }:
    flakeutils.lib.eachDefaultSystem (system:
      let
        pkgs        = (import nixpkgs) { inherit system; };
        naersk'     = pkgs.callPackage naersk {};
	cargoConfig = builtins.fromTOML(builtins.readFile(./Cargo.toml));
	name        = cargoConfig.package.name;
        # pre-commit  = pre-commit-hooks.lib."${system}".run;

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
        

#        checks = {
 #         build = self.defaultPackage.${system};
  #        pre-commit-check = pre-commit {
   #         src = ./.;
    #        hooks = {
#              nixfmt.enable = true;
 #             rustfmt.enable = true;
  #            cargo-check.enable = true;
   #         };
    #      };
     #   };
   	
	packages.default = packages.${name};
	defaultPackage = packages.${name};
	
	apps.wash = flakeutils.lib.mkApp {
          drv = packages.wash;
        };
        defaultApp = apps.wash;
      });
}
