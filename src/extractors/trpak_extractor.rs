use std::path::Path;
use flatbuffers::Vector;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write, BufReader, BufRead};

use crate::cli::State;
use crate::error_handler::SVExtractorError;
use crate::schemas::trpak_generated::trpak::*;
use crate::extractors::trpfs_extractor::*;

pub fn extract(state: &mut State, file_str: &String) -> Result<(), SVExtractorError> {
    write_files(state, &file_str)?;
    Ok(())
}

fn vector_to_vec64(vector: Vector<'_, u64>) -> Vec<u64> {
    let mut vec = Vec::<u64>::new();

    for i in 0..vector.len() {
        vec.push(vector.get(i));
    }

    return vec;
}

fn vector_to_vec8(vector: Vector<'_, u8>) -> Vec<u8> {
    let mut vec = Vec::<u8>::new();

    for i in 0..vector.len() {
        vec.push(vector.get(i));
    }

    return vec;
}

fn read_rtpak<'a>(trpak_path: &String, trpak_buffer: &'a mut Vec::<u8>) -> Result<TRPAK<'a>, SVExtractorError> {
    // Parse the rtpak and deserialize it with flatbuffers
    let mut trpak_reader = BufReader::new(File::open(&trpak_path)?);
    trpak_reader.read_to_end(trpak_buffer)?;
    
    return Ok(root_as_trpak(trpak_buffer)?);
}

fn get_file_names(state: &mut State) -> Result<Vec<String>, SVExtractorError> {
    let trpak_names_file = BufReader::new(File::open(format!("{}/names_trpak.txt",&state.info))?).lines();
    Ok(trpak_names_file.collect::<Result<_, _>>()?)
}

fn write_files(state: &mut State, file_path: &String) -> Result<(), SVExtractorError> {
    // Parse the rtpak and deserialize it with flatbuffers
    let mut trpak_buffer = Vec::<u8>::new();

    let trpak = read_rtpak(file_path, &mut trpak_buffer)?;
    let trpak_hashes = vector_to_vec64(trpak.clone().hashes().unwrap());
    let trpak_files = trpak.clone().files().unwrap();

    let names_file = get_file_names(state)?;

    for i in 0..trpak_files.len() {
        let unhashed = names_file.iter().find(|n| trpak_hashes[i] == fnv1a64(&n, &mut state.hash_dict));
        let new_file: _;
        match unhashed {
            Some(nohash) => new_file = Path::new(&format!("{}/arc/{}", &state.output, nohash)).to_str().unwrap().to_string(),
            None => new_file = Path::new(&file_path).parent().unwrap().join(format!("{:x}",trpak_hashes[i])).to_str().unwrap().to_string(),
        }
        
        let mut data = vector_to_vec8(trpak_files.get(i).data().clone().unwrap());
        let mut out = vec![0u8; trpak_files.get(i).decompressed_size().try_into()?];

        if trpak_files.get(i).compression_type() == Compression::OODLE {
            unsafe{
                let result = ooz_sys::Kraken_Decompress(data.as_ptr(), data.len(), out.as_mut_ptr(), out.len());
                
                if result != trpak_files.get(i).decompressed_size() as i32 {
                    return Err(SVExtractorError::OodleDecompressError)
                }
                data = out;
            }
        }

        if !Path::new(&new_file).exists() {
            create_dir_all(Path::new(&new_file).parent().unwrap())?;
        }

        let mut out_file = File::create(new_file)?;
        out_file.write_all(&data)?;
    }

    Ok(())
}