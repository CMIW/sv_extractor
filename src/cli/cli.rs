use std::env;
use std::path::Path;
use serde::{Deserialize, Serialize};
use clap::{Parser, ValueEnum};
use std::collections::HashMap;

/// The arguments for the CLI clap make things really easy
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[arg(short, long)]
	pub romfs: String,
	
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
	// TRPAK Extraction
	//TRPAK,
	// Full Extraction (Note: this might take a while)
	//Full_TRPFS,
}

/// CLI state that holds all the important information, like paths and variables that I
/// don't want them to be global
#[derive(Debug, Default)]
pub struct State {
	pub romfs: String,
	pub output: String,
	pub schemas: String,
	pub info: String,
	pub flatc: String,
	pub trpfs: String,
	pub trpfd: String,
	pub fs_trpfs: String,
	pub names_dict: HashMap<String, String>,
	pub hash_dict: HashMap<String, u128>,
	pub init_offset: u64,
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
			romfs: args.romfs.clone(), 
			output: args.output.clone(), 
			schemas: env::current_dir().unwrap().join("schemas").to_str().unwrap().to_string(),
			flatc: flatc,
			info: env::current_dir().unwrap().join("info").to_str().unwrap().to_string(),
			trpfs: Path::new(&args.romfs).join("arc/data.trpfs").to_str().unwrap().to_string(),
    		trpfd: Path::new(&args.romfs).join("arc/data.trpfd").to_str().unwrap().to_string(),
    		fs_trpfs: env::current_dir().unwrap().join("info/fs_data_separated.trpfs").to_str().unwrap().to_string(),
    		names_dict: HashMap::new(),
    		hash_dict: HashMap::new(),
    		..Default::default()
		}
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