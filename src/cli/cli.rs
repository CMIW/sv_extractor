use std::{env, fs};
use std::path::Path;
use clap::{Parser, ValueEnum};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::error_handler::SVExtractorError;

/// The arguments for the CLI clap make things really easy
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long)]
	pub romfs: Option<String>,

	#[arg(short, long)]
	pub trpak: Option<String>,
	
	#[arg(short, long,default_value_t = env::current_dir().unwrap().join("output").to_str().unwrap().to_string())]
	pub output: String,
	
	#[arg(short, long, value_enum)]
	pub extraction: ExtractionOption,
}

/// The three types of extraction that the tool supports
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExtractionOption{
	/// TRPFS Extraction
	TRPFS,
	/// TRPAK Extraction
	TRPAK,
	// Full Extraction (Note: this might take a while)
	FULL,
}

/// CLI state that holds all the important information, like paths and variables that I
/// don't want them to be global
#[derive(Debug, Default)]
pub struct State {
	pub info: String,
	pub romfs: String,
	pub flatc: String,
	pub trpfs: String,
	pub trpfd: String,
	pub output: String,
	pub schemas: String,
	pub oo2core: String,
	pub fs_trpfs: String,
	pub init_offset: u64,
	pub hash_dict: HashMap<String, u128>,
	pub names_dict: HashMap<String, String>,
}

impl State {
	pub fn new(args: &Args) -> Self {
		let flatc; 
	    if cfg!(target_os = "windows") {
	        flatc = env::current_dir().unwrap().join("bin/flatc.exe").to_str().unwrap().to_string()
	    }
	    else {
	        flatc = env::current_dir().unwrap().join("bin/./flatc").to_str().unwrap().to_string()
	    }
	    
	    State { 
			output: args.output.clone(),
			schemas: env::current_dir().unwrap().join("schemas").to_str().unwrap().to_string(),
			flatc: flatc,
			info: env::current_dir().unwrap().join("info").to_str().unwrap().to_string(),
			fs_trpfs: env::current_dir().unwrap().join("info/fs_data_separated.trpfs").to_str().unwrap().to_string(),
    		names_dict: HashMap::new(),
    		hash_dict: HashMap::new(),
    		..Default::default()
		}
	}

	pub fn add_romfs(&mut self, args: &Args) -> &Self {
		let romfs_dinding = args.romfs.as_ref().unwrap().to_string();

		self.romfs = romfs_dinding.clone(); 
		self.trpfs = Path::new(&romfs_dinding.clone()).join("arc/data.trpfs").to_str().unwrap().to_string();
    	self.trpfd = Path::new(&romfs_dinding.clone()).join("arc/data.trpfd").to_str().unwrap().to_string();
    	
    	self
	}

	// Searches every entry in the /bin directory looking for a oo2core*.dll
	pub fn find_oo2core(&mut self) -> Result<(), SVExtractorError> {
		let current_dir = env::current_dir()?.join("bin");
		for entry in fs::read_dir(current_dir)? {
			let file_name_binding = entry.as_ref().unwrap().file_name();
			let file_name = file_name_binding.to_str().unwrap();
			if file_name.contains("oo2core") && file_name.contains(".dll") {
				self.oo2core = entry?.path().as_path().to_str().unwrap().to_string();
				return Ok(());
			}
		}
		Err(SVExtractorError::Missingoo2core)
	}
}

/// The next structs are used to deserialize the JSON files that are extracted from 
/// the data.trpfs and data.trpfd files

/// Taken from schemas/trpfs.fbs
#[derive(Debug, Deserialize, Serialize)]
pub struct TRPFS {
	pub hashes: Vec<u64>,
	pub file_offsets: Vec<u64>,
}

/// Taken from schemas/trpfd.fbs
#[derive(Debug, Deserialize, Serialize)]
pub struct TRPFD {
	pub file_hashes: Vec<u64>,
	pub paths: Vec<String>,
	map: Vec<MapTable>,
	info: Vec<InfoTable>,
}

/// Taken from schemas/trpfd.fbs
#[derive(Debug, Deserialize, Serialize)]
struct MapTable {
	index: u32,
	unk2: EmptyTable,
}

/// Taken from schemas/trpfd.fbs
#[derive(Debug, Deserialize, Serialize)]
struct EmptyTable {

}

/// Taken from schemas/trpfd.fbs
#[derive(Debug, Deserialize, Serialize)]
struct InfoTable {
	size: u64,
	count: u64,
}

/// Taken from schemas/trpak.fbs
#[derive(Debug, Deserialize, Serialize)]
pub struct TRPAK {
	pub hashes: Vec<u64>,
	pub files: Vec<File>,
}

/// Taken from schemas/trpak.fbs
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
	unused: u8,
	pub compression_type: Compression,
	unk1: u8,
	pub decompressed_size: u64,
	pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Compression {
	OODLE = 3,
	NONE = 255,
}