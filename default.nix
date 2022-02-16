let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "1.1.0";

	src = ./.;

	cargoSha256 = "10s59wfdzl7rghmkm9pnypvllhwph7k0aykhj5xpyrc0xhpkyzkn";
}
