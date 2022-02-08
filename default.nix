let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "1.0.0";

	src = ./.;

	cargoSha256 = "0j13ky96qy9drp7v8mpki2r3amm83cxx02afnz66k3s6qjqpivy7";
}
