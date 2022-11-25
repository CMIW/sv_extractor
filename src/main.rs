use clap::Parser;
use std::process;
use std::path::Path;
use sv_extractor::extractors::*;
use sv_extractor::error_handler::SVExtractorError;
use sv_extractor::cli::{Args, ExtractionOption, State};

/// This tool was created to make extracting data from SV binary files easier by making
/// an executable CLI instead of a python script. It should also support both windows 
/// and linux.
fn main() {
    // Parse the the arguments for the CLI
    let args = Args::parse();

    // Create a new app state
    let mut state = State::new(&args);

    // Verify that the romfs, .trpak path exists
	if !Path::new(&args.path).exists() {
		eprintln!("{}", SVExtractorError::NoDirectory{path: args.path.clone()});
		process::exit(1);
	}

    match args.extraction {
    	ExtractionOption::TRPFS => { 
    		state.add_romfs(&args.path);

    		// Validate that the needed paths exist
		    if !Path::new(&state.trpfs).exists() {
		        eprintln!("{}", SVExtractorError::NoDirectory{path: state.trpfs});
		        process::exit(1);
		    }
		    if !Path::new(&state.trpfd).exists() {
		        eprintln!("{}", SVExtractorError::NoDirectory{path: state.trpfd});
		        process::exit(1);
		    }

		    // Execute the extraction
    		trpfs_extractor::extract(&mut state).unwrap_or_else(|err| {
		        process_errors(err);
		        process::exit(1);
		    }); 
    	},
    	ExtractionOption::TRPAK => { 
    		if !Path::new(&args.path).is_file() {
				eprintln!("{}", SVExtractorError::IsADirectory{path: args.path.clone()});
				process::exit(1);
			}
    		
    		// Execute the extraction
    		trpak_extractor::extract(&args.path).unwrap_or_else(|err| {
		        process_errors(err);
		        process::exit(1);
		    }); 
    	},
    	ExtractionOption::FULL => {
    		state.add_romfs(&args.path);

    		// Validate that the needed paths exist
		    if !Path::new(&state.trpfs).exists() {
		        eprintln!("{}", SVExtractorError::NoDirectory{path: state.trpfs});
		        process::exit(1);
		    }
		    if !Path::new(&state.trpfd).exists() {
		        eprintln!("{}", SVExtractorError::NoDirectory{path: state.trpfd});
		        process::exit(1);
		    }

		    full_extractor::extract(&mut state).unwrap_or_else(|err| {
		        process_errors(err);
		        process::exit(1);
		    }); 
    	},
    }

    println!("Extraction complete!");
    
}

// Handle some especific errors
fn process_errors(err: SVExtractorError){
	match err {
		SVExtractorError::InvalidFlatbuffer(_) => eprintln!("Invalid file, expected: data.trpfs, data.trpfd or *.tprak from Pokemon Scarlet or Violet"),
		_ => eprintln!("{}", err),
	}
}