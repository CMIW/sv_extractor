/// To extract the .trpfs we read the u64 pointer as a start offset form the second 
/// 8 bytes of the file. Then we read the u64 pointer as a end of file offset form 
/// the last 8 bytes of the file. Those two offsets are used to extract the content 
/// of the .trpfs file to a new .trpfs file without the header. 
/// The ~6GB files are flatbuffers.

use std::path::Path;
use flatbuffers::Vector;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use std::fs::{File, create_dir_all};
use std::io::{SeekFrom, BufReader, BufRead};

use crate::cli::{State};
use crate::error_handler::SVExtractorError;
use crate::schemas::trpfs_generated::trpfs::*;
use crate::schemas::trpfd_generated::trpfd::*;

const FS_MAGIC: &str = "ONEPACK";

pub fn extract(state: &mut State) -> Result<(), SVExtractorError> {
    extract_trpfs(state)?;
    extract_trpfd(state)?;
    write_files(state)?;
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

    let mut f1 = File::create(&state.fs_trpfs)?;
    f1.write_all(&content_buffer)?;
	
	Ok(())
}

fn extract_trpfd(state: &mut State) -> Result<(), SVExtractorError> {
    println!("Extracting data from trpfd file...");
    let onames_file = BufReader::new(File::open(format!("{}/names_original.txt",&state.info))?).lines();
    let cnames_file = BufReader::new(File::open(format!("{}/names_changed.txt",&state.info))?).lines();
    
    for (onames, cnames) in onames_file.zip(cnames_file){
        state.names_dict.insert(onames.unwrap(), cnames.unwrap());
    }

    Ok(())
}

fn vector_to_vec(vector: Vector<'_, u64>) -> Vec<u64> {
    let mut vec = Vec::<u64>::new();

    for i in 0..vector.len() {
        vec.push(vector.get(i));
    }

    return vec;
}

// Reads the .rtpfs file and deserialize it into a TRPFS struct
fn read_rtpfs<'a>(rtpfs_path: &String, trpfs_buffer: &'a mut Vec::<u8>) -> Result<TRPFS<'a>, SVExtractorError> {
    // Parse the rtpfs and deserialize it with flatbuffers
    let mut trpfs_reader = BufReader::new(File::open(&rtpfs_path)?);
    trpfs_reader.read_to_end(trpfs_buffer)?;
    
    return Ok(root_as_trpfs(trpfs_buffer).unwrap());
}

// Reads the .rtpfd file and deserialize it into a TRPFd struct
fn read_rtpfd<'a>(rtpfd_path: &String, trpfd_buffer: &'a mut Vec::<u8>) -> Result<TRPFD<'a>, SVExtractorError> {
    // Parse the rtpfs and deserialize it with flatbuffers
    let mut trpfd_reader = BufReader::new(File::open(&rtpfd_path)?);
    trpfd_reader.read_to_end(trpfd_buffer)?;

    let trpfd = root_as_trpfd(trpfd_buffer).unwrap();
    
    return Ok(trpfd);
}

// Writes all the files decompressed from the .rtpfs and .rtpfd files
fn write_files(state: &mut State) -> Result<(), SVExtractorError> {
    println!("Extracting files to {} ...", &state.output);
    let mut data_reader = BufReader::new(File::open(&state.trpfs)?);

    // Parse the rtpfs and deserialize it with flatbuffers
    let mut trpfs_buffer = Vec::<u8>::new();
    let trpfs = read_rtpfs(&state.fs_trpfs, &mut trpfs_buffer)?;
    // Get the hashes
    let trpfs_hashes = vector_to_vec(trpfs.clone().hashes().unwrap());
    // Get the file offsets
    let mut trpfs_offsets = vector_to_vec(trpfs.clone().file_offsets().unwrap());

    // We get the len here because later will append the init offset to the list and 
    // read an offest ahead, this avoid an index out of range error and we also the 
    // the end of file offset to write the last file
    let file_count = trpfs_offsets.len();

    trpfs_offsets.push(state.init_offset);
    
    // Parse the rtpfd and deserialize it with flatbuffers
    let mut trpfd_buffer = Vec::<u8>::new();
    let trpfd = read_rtpfd(&state.trpfd, &mut trpfd_buffer)?;


    for i in 0..file_count {
        let offset = trpfs_offsets[i];
        let end_offset = trpfs_offsets[i + 1];
        let name_hash = trpfs_hashes[i];

        // Just in case the there is no path 
        let mut path: Option<String> = None;
        for j in &trpfd.paths().unwrap() {
            if name_hash == fnv1a64(j, &mut state.hash_dict) {
                if state.names_dict.contains_key(j) {
                    path = Some(format!("{}/{}", &state.output, &state.names_dict[j]));
                }
                else {
                    path = Some(format!("{}/{}", &state.output, j));
                }
                break;
            }
        }
        // Create the output file and write to it, if the path is empty ignore it
        match path {
            Some(path_str) => {
                // Create the parent path if it doesn't exists
                if !Path::new(&path_str).exists() {
                    create_dir_all(Path::new(&path_str).parent().unwrap())?;
                }

                let mut out_file_buff = vec![0u8; (end_offset - offset).try_into()?];
                data_reader.seek(SeekFrom::Start(offset.try_into()?))?;
                data_reader.read_exact(&mut out_file_buff)?;

                let mut out_file = File::create(path_str)?;
                out_file.write_all(&out_file_buff)?;
            }
            None => {  }
        }
    
    }
    Ok(())
}

fn fnv1a64(_str: &str, hash_map: &mut HashMap<String, u128>) -> u64 {
    if hash_map.contains_key(_str) {
        return hash_map[_str].try_into().unwrap();
    }

    let fnv_prime: u128 = 1099511628211;
    let mut offset_basis: u128 = 14695981039346656837;

    for i in _str.chars() {
        offset_basis ^= i as u128; // apply a bitwise XOR
        offset_basis = (offset_basis * fnv_prime) % (2_u128.pow(64));
    }

    hash_map.insert(_str.to_string(), offset_basis);

    offset_basis.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a64_hash_test(){
        let mut hash_dict: HashMap<String, u128> = HashMap::new();
    	let value = "pm0081_00_00_20146_stepout01.traef";
        let _fnv1a64_res: u64 = 8206631493468059913;
        let fnv1a64_hex: u64 = 0x71e3cfdcdab27909;

        assert_eq!(fnv1a64_hex, fnv1a64(value, &hash_dict));
	}
}
