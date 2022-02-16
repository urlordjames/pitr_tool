let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "1.0.1";

	src = ./.;

	cargoSha256 = "1w3z6qhly2f1lm7irh1xqrp6afzh2g9fd96784lk9qyzvknxa815";
}
