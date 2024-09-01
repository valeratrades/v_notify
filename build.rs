use std::{env, fs, path::Path, process::Command};

fn main() {
	let tg_path = Path::new("/usr/local/bin/tg");

	// Always check if tg exists and rerun the script if it doesn't
	println!("cargo:rerun-if-not-exist=/usr/local/bin/tg");

	if !tg_path.exists() {
		println!("cargo:warning=tg not found, attempting to build and install...");

		// Clone the repository
		if let Err(e) = Command::new("git").args(["clone", "https://github.com/valeratrades/tg"]).status() {
			println!("cargo:warning=Failed to clone repository: {}", e);
			return;
		}

		// Change directory
		if let Err(e) = env::set_current_dir("tg") {
			println!("cargo:warning=Failed to change directory: {}", e);
			return;
		}

		// Build the project
		if let Err(e) = Command::new("cargo").args(["build", "--release"]).status() {
			println!("cargo:warning=Failed to build project: {}", e);
			return;
		}

		// Copy the binary using sudo
		println!("cargo:warning=Attempting to copy tg to /usr/local/bin. You may be prompted for your password.");
		if let Err(e) = Command::new("sudo").args(["cp", "target/release/tg", "/usr/local/bin/"]).status() {
			println!(
				"cargo:warning=Failed to copy binary: {}. You may need to manually copy the binary to /usr/local/bin with sudo privileges.",
				e
			);
			return;
		}

		// Clean up
		if let Err(e) = env::set_current_dir("..") {
			println!("cargo:warning=Failed to change back to parent directory: {}", e);
		}
		if let Err(e) = fs::remove_dir_all("tg") {
			println!("cargo:warning=Failed to remove cloned directory: {}", e);
		}

		println!("cargo:warning=tg has been built and installed to /usr/local/bin/tg");
	}
}
