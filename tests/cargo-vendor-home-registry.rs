use libroast::common::Compression;
use obs_service_cargo_vendor_home_registry::{
	cli::HomeRegistryArgs,
	opts::run_vendor_home_registry,
};
use rand::prelude::*;
use std::{
	io,
	path::PathBuf,
};
use test_log::test;
use tokio::fs;
use tokio_test::{
	assert_ok,
	task::spawn,
};
use tracing::{
	error,
	info,
};

const MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR", "No such manifest dir");

#[test]
fn cargo_vendor_home_registry_cargo_vendor_home_registry() -> io::Result<()>
{
	let mut rng = rand::thread_rng();
	let random_tag: u8 = rng.gen();
	let random_tag = random_tag.to_string();
	let outdir = PathBuf::from("/tmp");
	let mut registry = HomeRegistryArgs {
		target: PathBuf::from(MANIFEST_DIR),
		compression: Compression::default(),
		tag: Some(random_tag),
		outdir,
		manifest_paths: vec![],
		i_accept_the_risk: vec![],
		update: false,
		custom_root: None,
		no_root_manifest: false,
		triple: vec![],
		ignore_rust_version: false,
	};

	let res = run_vendor_home_registry(&registry);
	if res.is_err()
	{
		info!("Possible that it needs to be updated.");
		registry.update = true;
		assert_ok!(run_vendor_home_registry(&registry));
	}
	else
	{
		assert_ok!(run_vendor_home_registry(&registry));
	}

	Ok(())
}

#[test(tokio::test)]
async fn monorepo_test_1() -> io::Result<()>
{
	let source = "https://github.com/ibm-s390-linux/s390-tools/archive/refs/tags/v2.29.0.tar.gz";
	let mut rng = rand::thread_rng();
	let random_tag: u8 = rng.gen();
	let random_tag = random_tag.to_string();
	let response = reqwest::get(source).await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::ConnectionAborted, err)
	})?;
	let fname = response
		.url()
		.path_segments()
		.and_then(|segments| segments.last())
		.and_then(|name| {
			if name.is_empty()
			{
				None
			}
			else
			{
				Some(name)
			}
		})
		.unwrap_or("balls");
	info!("Source file: {}", &fname);
	let outfile = format!("/{}/{}", "tmp", &fname);
	info!("Downloaded to: '{:?}'", &outfile);
	fs::File::create(&outfile).await.map_err(|err| {
		error!(?err);
		err.to_string();
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let outfile = PathBuf::from(&outfile);
	let data = response.bytes().await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let data = data.to_vec();
	fs::write(&outfile, data).await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let outdir = PathBuf::from("/tmp");
	let manifest_paths =
		vec![PathBuf::from("rust/utils/Cargo.toml"), PathBuf::from("rust/pv/Cargo.toml")];
	let mut registry = HomeRegistryArgs {
		target: outfile,
		compression: Compression::default(),
		tag: Some(random_tag),
		outdir,
		manifest_paths,
		i_accept_the_risk: vec![],
		update: false,
		custom_root: None,
		no_root_manifest: true,
		triple: vec![],
		ignore_rust_version: false,
	};
	let res = run_vendor_home_registry(&registry);
	if res.is_err()
	{
		info!("Possible that it needs to be updated.");
		registry.update = true;
		assert_ok!(run_vendor_home_registry(&registry));
	}
	else
	{
		assert_ok!(run_vendor_home_registry(&registry));
	}
	Ok(())
}

async fn vendor_home_registry_source(source: &str) -> io::Result<()>
{
	let mut rng = rand::thread_rng();
	let random_tag: u8 = rng.gen();
	let random_tag = random_tag.to_string();
	let response = reqwest::get(source).await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let fname = response
		.url()
		.path_segments()
		.and_then(|segments| segments.last())
		.and_then(|name| {
			if name.is_empty()
			{
				None
			}
			else
			{
				Some(name)
			}
		})
		.unwrap_or("balls");
	info!("Source file: {}", &fname);
	let outfile = format!("/{}/{}", "tmp", &fname);
	info!("Downloaded to: '{:?}'", &outfile);
	fs::File::create(&outfile).await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let outfile = PathBuf::from(&outfile);
	let data = response.bytes().await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let data = data.to_vec();
	fs::write(&outfile, data).await.map_err(|err| {
		error!(?err);
		io::Error::new(io::ErrorKind::InvalidData, err)
	})?;
	let outdir = PathBuf::from("/tmp");
	let mut registry = HomeRegistryArgs {
		target: outfile,
		compression: Compression::default(),
		tag: Some(random_tag),
		outdir,
		manifest_paths: vec![],
		i_accept_the_risk: vec![],
		update: false,
		custom_root: None,
		no_root_manifest: false,
		triple: vec![],
		ignore_rust_version: false,
	};
	let res = run_vendor_home_registry(&registry);
	if res.is_err()
	{
		info!("Possible that it needs to be updated.");
		registry.update = true;
		assert_ok!(run_vendor_home_registry(&registry));
	}
	else
	{
		assert_ok!(run_vendor_home_registry(&registry));
	}
	Ok(())
}

#[test(tokio::test)]
async fn vendor_home_registry_sources_with_workspace_configuration() -> io::Result<()>
{
	let sources: &[&str] = &[
		"https://github.com/zellij-org/zellij/archive/refs/tags/v0.40.1.tar.gz",
		"https://github.com/alacritty/alacritty/archive/refs/tags/v0.14.0.tar.gz",
	];
	for src in sources
	{
		let _ = spawn(async move {
			#[allow(clippy::unwrap_used)]
			vendor_home_registry_source(src).await.unwrap();
			src
		})
		.await;
	}
	Ok(())
}
