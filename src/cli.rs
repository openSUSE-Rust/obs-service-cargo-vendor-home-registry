use clap::Parser;
use libroast::common::Compression;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, author, name = "cargo vendor home registry")]
pub struct HomeRegistryArgs
{
	#[arg(
		long,
		short = 't',
		visible_aliases = ["src", "srctar", "srcdir"],
		help = "Target source directory or tarball to vendor cargo home registry."
	)]
	pub target: PathBuf,
	#[arg(long, value_enum, short = 'c', default_value_t, help = "Set what compression to use.")]
	pub compression: Compression,
	#[arg(
		long,
		short = 'T',
		help = "Whether to add a tag after the name \"registry\" appended with a \"-\". Useful if you \
		        plan to set `CARGO_HOME` in different contexts."
	)]
	pub tag: Option<String>,
	#[arg(long, short = 'd', help = "Directory to put where the vendored registry home tarball.")]
	pub outdir: PathBuf,
	#[arg(
		long,
		short = 'm',
		help = "Additional manifests paths. Good if you want to use other manifest paths and if you \
		        explicitly set `--no_root_manifest`."
	)]
	pub manifest_paths: Vec<PathBuf>,
	#[arg(
		long,
		help = "A list of rustsec-id's to ignore. By setting this value, you acknowledge that this \
		        issue does not affect your package and you should be exempt from resolving it."
	)]
	pub i_accept_the_risk: Vec<String>,
	#[arg(
		long,
		short = 'u',
		default_value_t = true,
		action = clap::ArgAction::Set,
		help = "Whether to update the dependencies or not. ⚠️ Be careful with setting this\
    			because a dependency might not follow semver and might introduce breaking changes.")]
	pub update: bool,
	#[arg(
		long,
		short = 'C',
		help = "Whether you want to manually set the root of the project. Useful with a combination \
		        with `--manifest-paths` or `--no-root-manifest`."
	)]
	pub custom_root: Option<String>,
	#[arg(
		long,
		short = 'N',
		default_value_t = false,
		action = clap::ArgAction::Set,
		help = "If a project has no root manifest, this flag is useful for those situations to set \
    			the manifest path manually. Useful in combination with `--manifest-paths` flag.")]
	pub no_root_manifest: bool,
	#[arg(
		long,
		short = 'a',
		help = "Specify target triple. You can check out the list by running `rustc --print target-list`. \
    			See more in the following documentation - <https://doc.rust-lang.org/cargo/guide/build-cache.html>."
	)]
	pub triple: Vec<String>,
	#[arg(
		long,
		short = 'R',
		default_value_t = true,
		action = clap::ArgAction::Set,
		help = "Whether to pass the `--ignore-rust-version` flag when generating the lockfile.")]
	pub ignore_rust_version: bool,
}
