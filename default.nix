let pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage {
	pname = "pitr_tool";
	version = "1.1.1";

	src = ./.;

	cargoSha256 = "1r48w4z7q8q3rclcjk5zq7275lj1gjq71aiz37pv1wdanv8jkd6f";
}
