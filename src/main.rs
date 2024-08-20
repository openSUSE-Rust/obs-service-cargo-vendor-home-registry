use obs_service_cargo_vendor_home_registry::cli::*;
use std::{io, io::IsTerminal};

use terminfo::{capability, Database};

use tracing::{debug, info, trace, warn, Level};

use clap::Parser;

use tracing_subscriber::filter;

fn main() -> Result<(), VendorError> {
	let args = VendorArgs::parse();
	if let Ok(manifest_options) = args.generate_manifest_options() {
		manifest_options.iter().for_each(|(key, val)| {
			println!("Path to manifest: {}\nUpdate: {}", key.display(), val);
		});
	}
	Ok(())
}
