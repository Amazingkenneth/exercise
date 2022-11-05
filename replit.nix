{ pkgs }: {
	deps = [
		pkgs.gh
		pkgs.rustup
		pkgs.cargo-edit
			pkgs.rust-analyzer
		pkgs.clang_13
		pkgs.ccls
		pkgs.gdb
		pkgs.gnumake
	];
}
