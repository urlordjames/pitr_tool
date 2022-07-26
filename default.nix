let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "1.2.0";

	src = ./.;

	cargoLock = {
		lockFile = ./Cargo.lock;
		outputHashes = {
			"pitr_util-1.2.0" = "sha256-NhYVoysitwb7/CTDBuDDo2hX7PvnfRo2dF/hq1hCI+8=";
		};
	};
}
