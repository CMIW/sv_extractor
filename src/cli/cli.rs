use std::{env};
use std::path::Path;
use clap::{Parser, ValueEnum};
use std::collections::HashMap;

/// The arguments for the CLI clap make things really easy
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	/// The path to the romfs folder or .trpak file 
	#[arg(short, long)]
	pub path: String,
	
	/// The path were the extracted files should be placed
	#[arg(short, long, default_value_t = env::current_dir().unwrap().join("output").to_str().unwrap().to_string())]
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
	/// Full Extraction (Note: this will take a while)
	FULL,
}

/// CLI state that holds all the important information, like paths and variables that I
/// don't want them to be global
#[derive(Debug, Default)]
pub struct State {
	pub info: String,
	pub romfs: String,
	pub trpfs: String,
	pub trpfd: String,
	pub output: String,
	pub fs_trpfs: String,
	pub init_offset: u64,
	pub hash_dict: HashMap<String, u128>,
	pub names_dict: HashMap<String, String>,
}

impl State {
	pub fn new(args: &Args) -> Self {
		State { 
			output: args.output.clone(),
			info: env::current_dir().unwrap().join("info").to_str().unwrap().to_string(),
			fs_trpfs: env::current_dir().unwrap().join("info/fs_data_separated.trpfs").to_str().unwrap().to_string(),
    		names_dict: HashMap::new(),
    		hash_dict: HashMap::new(),
    		..Default::default()
		}
	}

	pub fn add_romfs(&mut self, path: &str) -> &Self {
		self.romfs = path.to_string(); 
		self.trpfs = Path::new(path).join("arc/data.trpfs").to_str().unwrap().to_string();
    	self.trpfd = Path::new(path).join("arc/data.trpfd").to_str().unwrap().to_string();
    	
    	self
	}
}
