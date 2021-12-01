let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "0.1.0";

	src = ./.;

	cargoSha256 = "1q573hh30i8ngl92km8wwgj66cxknzqsn8ngmvwphgfiwy24n76g";
}
