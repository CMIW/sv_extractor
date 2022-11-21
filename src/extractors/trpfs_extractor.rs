use std::path::Path;
//use serde_json::{Value};
use std::process::Command;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use std::io::{SeekFrom, BufReader, BufRead};
use std::fs::{File, create_dir_all, read_to_string};

use crate::cli::{State, TRPFS, TRPFD};
use crate::error_handler::SVExtractorError;

const FS_MAGIC: &str = "ONEPACK";

pub fn extract(mut state: State) -> Result<(), SVExtractorError> {
    extract_trpfs(&mut state)?;
    extract_trpfs_flatc(&state)?;
    extract_trpfd_flatc(&state)?;
    extract_trpfd(&mut state)?;
    write_files(&state)?;
    Ok(())
}

fn extract_trpfs(state: &mut State) -> Result<(), SVExtractorError> {
    println!("Extracting data from trpfs file...");
	// Open the file and create the buffer on where to load the bytes
    let f = File::open(&state.trpfs)?;
    let mut reader = BufReader::new(f);

    // Read the file header
    let mut magic_buf = vec![0u8; 8];
    reader.read_exact(&mut magic_buf)?;
    // The file that we are looking for has "ONEPACK" as the first 7 
    // bytes, so we read the first 8 bytes from the buffer then we 
    // remove the last byte that just holds an "end of line" character 
    let magic = std::str::from_utf8(&magic_buf[..7]).unwrap();

    // validate the file header
    if magic != FS_MAGIC {
        return Err(SVExtractorError::InvalidHeader{ 
            expected: FS_MAGIC.to_string(), 
            found: magic.to_string()
        });
    }

    // To get the init_offset we read a u64 pointer 
	let mut init_buf = [0u8; 8];
	reader.read_exact(&mut init_buf)?;
	let init_offset = u64::from_ne_bytes(init_buf);
    // store it for latter
    state.init_offset = init_offset;
	// No endianess or little endian works
	// let init_offset = u64::from_le_bytes(init_buf);

	// Set the file's current position at an offset of 0 relative to 
	// the end of the file then we set that pointer as the eof_offset
	reader.seek(SeekFrom::End(0))?;
	let eof_offset = reader.stream_position()?;

	// Move back to the start of the file content (after the header) and 
    // read the entire data
	reader.seek(SeekFrom::Start(init_offset.try_into()?))?;
	let content_size = eof_offset - init_offset;
	let mut content_buffer = vec![0u8; content_size.try_into()?];
	reader.read_exact(&mut content_buffer)?;

    // create the output file and write to it
    if !Path::new(&state.fs_trpfs).exists() {
        create_dir_all(Path::new(&state.fs_trpfs).parent().unwrap())?;
    }
	let mut f1 = File::create(&state.fs_trpfs)?;
    f1.write_all(&content_buffer)?;
	
	Ok(())
}

fn extract_trpfd(state: &mut State) -> Result<(), SVExtractorError> {
    println!("Extracting data from trpfd file...");
    let onames_file = BufReader::new(File::open(format!("{}/names_original.txt",&state.info))?).lines();
    let cnames_file = BufReader::new(File::open(format!("{}/names_changed.txt",&state.info))?).lines();
    
    for (onames, cnames) in onames_file.zip(cnames_file){
        state.names_hash.insert(onames.unwrap(), cnames.unwrap());
    }

    Ok(())
}

fn write_files(state: &State) -> Result<(), SVExtractorError> {
    println!("Extracting files...");
    let trpfd_str = read_to_string(format!("{}/data.json",&state.info))?;
    let trpfs_str = read_to_string(format!("{}/fs_data_separated.json",&state.info))?;

    let trpfd: TRPFD = serde_json::from_str(&trpfd_str)?;
    let mut trpfs: TRPFS = serde_json::from_str(&trpfs_str)?;

    let file_count = trpfs.file_offsets.len();

    trpfs.file_offsets.push(state.init_offset);

    for i in 0..file_count {
        let offset = trpfs.file_offsets[i];
        let end_offset = trpfs.file_offsets[i + 1];
        let name_hash = trpfs.hashes[i];
    }

    Ok(())
}

fn extract_trpfs_flatc(state: & State) -> Result<(), SVExtractorError> {
    // Set the paths arguments for the flatc tool
    let trpfs_schema = format!("{}/trpfs.fbs",&state.schemas);
    
    // Execute flatc
    Command::new(&state.flatc)
    .arg("--raw-binary")
    .arg("-o").arg(&state.info)
    .arg("--strict-json")
    .arg("--defaults-json")
    .arg("-t").arg(trpfs_schema)
    .arg("--").arg(&state.fs_trpfs)
    .spawn()?;

    Ok(())
}

fn extract_trpfd_flatc(state: &State) -> Result<(), SVExtractorError> {
    // Set the paths arguments for the flatc tool
    let trpfd_schema = format!("{}/trpfd.fbs",&state.schemas);
    
    // Execute flatc
    Command::new(&state.flatc)
    .arg("--raw-binary")
    .arg("-o").arg(&state.info)
    .arg("--strict-json")
    .arg("--defaults-json")
    .arg("-t").arg(trpfd_schema)
    .arg("--").arg(&state.trpfd)
    .spawn()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn FNV1a64_hash_test(){
        let HASH_DICT: HashMap<String, u64> = HashMap::new();
    	let value = "pm0081_00_00_20146_stepout01.traef";
        let FNV1a64_res: u64 = 8206631493468059913;
        let FNV1a64_hex: u64 = 0x71e3cfdcdab27909;

        
        
	}
}
