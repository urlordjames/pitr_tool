let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "1w8nl9zy80dwwqlld4f82cjl0crkj5hk22dn1wlkid6a1q2nwypa";
}
