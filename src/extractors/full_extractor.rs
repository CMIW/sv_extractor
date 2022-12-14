use std::path::Path;
use crate::cli::{State};
use crate::error_handler::SVExtractorError;
use crate::extractors::{trpfs_extractor, trpak_extractor};

pub fn extract(state: &mut State) -> Result<(), SVExtractorError> {
	// Extract the data form the .trpfs and .trpfd files
    trpfs_extractor::extract(state)?;
    let files_dict = &state.names_dict.clone();
    println!("This will take a while, extracting .trpak files...");
    // For every extracted file decompress the .trpak
    for (_oname, cname) in files_dict {
    	let path = Path::new(&state.output).join(&cname).to_str().unwrap().to_string();
    	trpak_extractor::extract(state, &path)?;
    }
    Ok(())
}