use crate::cli::HomeRegistryArgs;
use libroast::{
	common::{Compression, SupportedFormat},
	operations::{
		cli::{RawArgs, RoastArgs},
		raw::raw_opts,
		roast::roast_opts,
	},
	utils::{self, copy_dir_all, is_supported_format, process_globs},
};
use tracing_subscriber::registry;
use std::{
	fs, io,
	path::{Path, PathBuf},
};
use tempfile;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

pub fn cargo_command(
	subcommand: &str,
	options: &[String],
	curdir: impl AsRef<Path>,
) -> io::Result<String> {
	let cmd = std::process::Command::new("cargo")
		.arg(subcommand)
		.args(options.iter())
		.current_dir(curdir.as_ref())
		.output()?;
	trace!(?cmd);
	let stdoutput = String::from_utf8_lossy(&cmd.stdout);
	let stderrput = String::from_utf8_lossy(&cmd.stderr);
	if !cmd.status.success() {
		error!(?stdoutput);
		error!(?stderrput);
		return Err(io::Error::new(io::ErrorKind::Interrupted, stderrput));
	};
	debug!(?stdoutput);
	debug!(?stderrput);
	// Return the output on success as this has the infor for .cargo/config
	Ok(stdoutput.to_string())
}

fn cargo_fetch(curdir: &Path) -> io::Result<String> {
	cargo_command("fetch", &["--locked".to_string()], curdir)
}

fn cargo_generate_lockfile(curdir: &Path) -> io::Result<String> {
	cargo_command("generate-lockfile", &[], curdir)
}

pub fn run_vendor_home_registry(registry: &HomeRegistryArgs) -> io::Result<()> {
	info!("🛖🏃📦 Starting Cargo Vendor Home Registry");
	let tempdir_for_home_registry_binding =
		tempfile::Builder::new().prefix(".cargo").rand_bytes(12).tempdir()?;
	let home_registry_path = &tempdir_for_home_registry_binding.path();
	let home_registry_path = home_registry_path.join(".cargo");
	debug!(?home_registry_path);
	std::env::set_var("CARGO_HOME", &home_registry_path);
	let tempdir_for_workdir_binding =
		tempfile::Builder::new().prefix(".workdir").rand_bytes(12).tempdir()?;
	let workdir = &tempdir_for_workdir_binding.path();
	debug!(?workdir);
	let target = utils::process_globs(&registry.target)?;
	if target.is_dir() {
		copy_dir_all(&target, workdir)?;
	} else if target.is_file() && is_supported_format(&target).is_ok() {
		let raw_args =
			RawArgs { target: target.to_path_buf(), outdir: Some(workdir.to_path_buf()) };
		raw_opts(raw_args, false)?;
	}

	let setup_workdir = {
		let dirs: Vec<Result<std::fs::DirEntry, std::io::Error>> =
			std::fs::read_dir(workdir)?.collect();
		debug!(?dirs, "List of files and directories of the workdir");
		if dirs.len() > 1 {
			debug!(?workdir);
			workdir.to_path_buf()
		} else {
			match dirs.into_iter().last() {
				Some(p) => match p {
					Ok(dir) => {
						if dir.path().is_dir() {
							debug!("{}", dir.path().display());
							// NOTE: return new workdir
							dir.path()
						} else {
							error!(
								?dir,
								"Tarball was extracted but got a file and not a possible top-level directory."
							);
							return Err(io::Error::new(
								io::ErrorKind::Interrupted,
								"No top-level directory found after tarball was extracted".to_string(),
							));
						}
					}
					Err(err) => {
						error!(?err, "Failed to read directory entry");
						return Err(err);
					}
				},
				None => {
					error!("This should be unreachable here");
					unreachable!();
				}
			}
		}
	};
	debug!(?setup_workdir);

	cargo_generate_lockfile(&setup_workdir)?;
	cargo_fetch(&setup_workdir)?;
	info!("👉🏻🗑️ Removing unneeded directories");
	let registry_src_dir = &home_registry_path.join("registry").join("src");
	let registry_bin_dir = &home_registry_path.join("bin");
	let registry_caches = [".global-cache", ".package-cache", ".package-cache-mutate"];
	if registry_src_dir.exists() {
		info!("🚮 Removing {}", registry_src_dir.display());
		fs::remove_dir_all(registry_src_dir)?;
		info!("🤯 Removed {}", registry_src_dir.display());
	}
	if registry_bin_dir.exists() {
		info!("🚮 Removing {}", registry_bin_dir.display());
		fs::remove_dir_all(registry_bin_dir)?;
		info!("🤯 Removed {}", registry_bin_dir.display());
	}
	// for ca in registry_caches {
	// 	let cache = &home_registry_path.join(ca);
	// 	if cache.exists() {
	// 		info!("🚮 Removing {}", cache.display());
	// 		fs::remove_file(cache)?;
	// 		info!("🤯 Removed {}", cache.display());
	// 	}
	// }
	let outfile = match &registry.tag {
		Some(v) => format!("registry-{}", v),
		None => "registry".to_string(),
	};
	let mut outfile = PathBuf::from(outfile);
	let extension = match &registry.compression {
		Compression::Gz => "tar.gz",
		Compression::Xz => "tar.xz",
		Compression::Zst => "tar.zst",
		Compression::Bz2 => "tar.bz",
		Compression::Not => "tar",
	};

	if !outfile.set_extension(extension) {
		return Err(io::Error::new(io::ErrorKind::Other, "Unable to set extension"));
	}
	let roast_args = RoastArgs {
		target: home_registry_path,
		include: None,
		exclude: None,
		additional_paths: None,
		outfile,
		outdir: Some(registry.outdir.to_path_buf()),
		preserve_root: true,
		reproducible: true,
		ignore_git: false,
		ignore_hidden: false,
	};
	roast_opts(&roast_args, false)?;
	info!("📦 Cargo Vendor Home Registry finished.");
	Ok(())
}
