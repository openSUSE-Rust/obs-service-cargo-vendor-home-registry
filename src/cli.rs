use clap::Parser;
use libroast::common::Compression;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, author, name = "cargo vendor home registry")]
pub struct HomeRegistryArgs
{
	#[arg(long, short = 't', visible_aliases = ["src"])]
	pub target: PathBuf,
	#[arg(long, value_enum, short = 'c', default_value_t)]
	pub compression: Compression,
	#[arg(long, short = 'T')]
	pub tag: Option<String>,
	#[arg(long, short = 'd')]
	pub outdir: PathBuf,
	#[arg(long, short = 'm')]
	pub manifest_paths: Vec<PathBuf>,
	#[arg(
		long,
		help = "A list of rustsec-id's to ignore. By setting this value, you acknowledge that this \
		        issue does not affect your package and you should be exempt from resolving it."
	)]
	pub i_accept_the_risk: Vec<String>,
	#[arg(long, short = 'u', default_value_t = true)]
	pub update: bool,
	#[arg(long, short = 'C')]
	pub custom_root: Option<String>,
	#[arg(long, short = 'N', default_value_t = false)]
	pub no_root_manifest: bool,
}
