let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "0i95cjfn947anprjzflq8m24qplwcf8ya98hrhns8k5k6syv0km9";
}
