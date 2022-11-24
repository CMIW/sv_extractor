use std::path::Path;
use flatbuffers::Vector;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write, BufReader};

use crate::error_handler::SVExtractorError;
use crate::schemas::trpak_generated::trpak::*;

pub fn extract(file_str: &String) -> Result<(), SVExtractorError> {
    write_files(&file_str)?;
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
    // Parse the rtpfs and deserialize it with flatbuffers
    let mut trpak_reader = BufReader::new(File::open(&trpak_path)?);
    trpak_reader.read_to_end(trpak_buffer)?;
    
    return Ok(root_as_trpak(trpak_buffer).unwrap());
}

fn write_files(file_path: &String) -> Result<(), SVExtractorError> {
    // Parse the rtpfs and deserialize it with flatbuffers
    let mut trpak_buffer = Vec::<u8>::new();

    let trpak = read_rtpak(file_path, &mut trpak_buffer)?;
    let trpak_hashes = vector_to_vec64(trpak.clone().hashes().unwrap());
    let trpak_files = trpak.clone().files().unwrap();

    for i in 0..trpak_files.len() {
        let new_file = Path::new(&file_path).with_extension("").join(format!("{:x}",trpak_hashes[i]));
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