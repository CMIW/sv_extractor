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
    //println!("{:#?}", state);

    match args.extraction {
    	ExtractionOption::TRPFS => { 
    		// Verify that the romfs path exists
		    if let Some(path) = &args.romfs {
		    	if !Path::new(path).exists() {
			    	eprintln!("{}", SVExtractorError::NotDir{path: path.clone()});
			    	process::exit(1);
			    }
		    }

		    state.add_romfs(&args);

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
    		trpfs_extractor::extract(&mut state).unwrap_or_else(|err| {
		        eprintln!("{}", err);
		        process::exit(1);
		    }); 
    	},
    	ExtractionOption::TRPAK => { 
    		// Vefiry that the .trpak path exists
		    if let Some(path) = &args.trpak {
		    	if !Path::new(path).exists() {
			    	eprintln!("{}", SVExtractorError::NotDir{path: path.clone()});
			    	process::exit(1);
			    }
		    }
		    
		    // Execute the extraction
    		trpak_extractor::extract(&args.trpak.unwrap()).unwrap_or_else(|err| {
		        eprintln!("{}", err);
		        process::exit(1);
		    }); 
    	},
    	ExtractionOption::FULL => {
    		// Verify that the romfs path exists
		    if let Some(path) = &args.romfs {
		    	if !Path::new(path).exists() {
			    	eprintln!("{}", SVExtractorError::NotDir{path: path.clone()});
			    	process::exit(1);
			    }
		    }

		    state.add_romfs(&args);

    		// Validate that the needed paths exist
		    if !Path::new(&state.trpfs).exists() {
		        eprintln!("{}", SVExtractorError::NotDir{path: state.trpfs});
		        process::exit(1);
		    }
		    if !Path::new(&state.trpfd).exists() {
		        eprintln!("{}", SVExtractorError::NotDir{path: state.trpfd});
		        process::exit(1);
		    }

		    full_extractor::extract(&mut state).unwrap_or_else(|err| {
		        eprintln!("{}", err);
		        process::exit(1);
		    }); 
    	},
    }

    println!("Extraction complete!");
    
}