use std::collections::HashMap;
use std::default::Default;
use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use quick_xml::de::DeError;
use quick_xml::de::Deserializer;
use quick_xml::se;
use quick_xml::se::Serializer;

use clap::builder::TypedValueParser as _;
use clap::clap_derive;
use clap::Parser;
use clap::ValueEnum;

use infer;

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct VendorArgs {
	// See https://github.com/clap-rs/clap/blob/f45a32ec2c1506faf319d914d985927ed47b0b5e/examples/typed-derive.rs#L24-L26
	#[arg(long)]
	pub manifest_options: Option<Vec<String>>,
	#[arg(long)]
	pub custom_home_root: Option<PathBuf>,
}

#[derive(Debug)]
pub enum VendorError {
	InvalidOption(String),
}

impl Display for VendorError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let msg = match self {
			VendorError::InvalidOption(msg) => format!("Invalid Option: {}", msg),
		};

		write!(f, "{}", msg)
	}
}

impl VendorArgs {
	pub fn generate_manifest_options(&self) -> Result<HashMap<PathBuf, bool>, VendorError> {
		let mut map: HashMap<PathBuf, bool> = HashMap::new();
		if let Some(manifest_opts) = &self.manifest_options {
			for el in manifest_opts {
				match el.split_once(",") {
					Some((k, v)) => {
						let key = Path::new(&k).to_path_buf();
						let val = match v.trim() {
							// Default is "" which sets update True
							"true" | "t" | "T" | "True" | "" => Ok(true),
							"false" | "f" | "F" | "False" => Ok(false),
							_ => return Err(VendorError::InvalidOption(v.to_string())),
						}?;
						map.insert(key, val);
					}
					None => {
						let key = Path::new(&el).to_path_buf();
						map.insert(key, true);
					}
				};
			}
		}
		Ok(map)
	}
}
