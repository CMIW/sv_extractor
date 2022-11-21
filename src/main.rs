use clap::Parser;
use std::process;
use std::path::Path;
use sv_extractor::cli::{Args, ExtractionOption, State};
use sv_extractor::error_handler::SVExtractorError;
use sv_extractor::extractors::*;

/// This tool was created to make extracting data from SV binary files easier by making
/// an executable CLI instead of a python script. It should also support both windows 
/// and linux.
fn main() {
    // Parse the the arguments for the CLI
    let args = Args::parse();

    // Verify that the romfs path exists
    if !Path::new(&args.romfs).exists() {
    	eprintln!("{}", SVExtractorError::NotDir{path: args.romfs});
    	process::exit(1);
    }

    // Create a new app state
    let state = State::new(&args);
    //println!("{:#?}", state);

    match args.extraction {
    	ExtractionOption::TRPFS => { 
    		// Validate that the needed paths exist
		    if !Path::new(&state.trpfs).exists() {
		        eprintln!("{}", SVExtractorError::NotDir{path: state.trpfs});
		        process::exit(1);
		    }
		    if !Path::new(&state.trpfd).exists() {
		        eprintln!("{}", SVExtractorError::NotDir{path: state.trpfd});
		        process::exit(1);
		    }

		    // Execute the extraction
    		trpfs_extractor::extract(state).unwrap_or_else(|err| {
		        eprintln!("{}", err);
		        process::exit(1);
		    }); 
    	},
    	//ExtractionOption::TRPAK => {},
    	//ExtractionOption::Full => {},
    }
}