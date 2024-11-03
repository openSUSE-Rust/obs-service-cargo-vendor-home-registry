use clap::Parser;
use obs_service_cargo_vendor_home_registry::{
	cli,
	opts,
};
use std::io;
#[allow(unused_imports)]
use tracing::{
	debug,
	error,
	info,
	trace,
	warn,
	Level,
};
fn main() -> io::Result<()>
{
	libroast::utils::start_tracing();
	info!("🛖🏃📦 Starting Cargo Vendor Home Registry");
	let home_registry = cli::HomeRegistryArgs::parse();
	opts::run_vendor_home_registry(&home_registry)?;
	info!("📦 Cargo Vendor Home Registry finished.");
	Ok(())
}
