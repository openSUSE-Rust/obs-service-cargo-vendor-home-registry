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
}
