use std::path::Path;
use std::io::{Write};
use std::process::Command;
use std::fs::{File, create_dir_all, read_to_string};

use crate::error_handler::SVExtractorError;
use crate::cli::{State, TRPAK, Compression};

pub fn extract(state: &mut State, file_str: &String) -> Result<(), SVExtractorError> {
    extract_trpak_flatc(state, &file_str)?;
    write_files(&file_str)?;
    Ok(())
}

// Run the flatc extraction command
fn extract_trpak_flatc(state: &State, file_str: &String) -> Result<(), SVExtractorError> {
    // Set the paths arguments for the flatc tool
    let trpak_schema = format!("{}/trpak.fbs",&state.schemas);
    let output = Path::new(&file_str).parent().unwrap();
    
    // Execute flatc
    Command::new(&state.flatc)
    .arg("--raw-binary")
    .arg("-o").arg(output)
    .arg("--strict-json")
    .arg("--defaults-json")
    .arg("-t").arg(&trpak_schema)
    .arg("--").arg(&file_str)
    .spawn()?.wait().unwrap();

    Ok(())
}

fn write_files(file_str: &String) -> Result<(), SVExtractorError> {
    let json_path = Path::new(&file_str).with_extension("json");
    let trpak_str = read_to_string(&json_path)?;
    let trpak: TRPAK = serde_json::from_str(&trpak_str)?;

    for i in 0..trpak.files.len() {
        let new_file = json_path.with_extension("").join(format!("{:x}",trpak.hashes[i]));
        let mut data = trpak.files[i].data.clone();
        let mut out = vec![0u8; trpak.files[i].decompressed_size.try_into()?];
        
        if trpak.files[i].compression_type == Compression::OODLE {
            unsafe{
                let result = ooz_sys::Kraken_Decompress(data.as_ptr(), data.len(), out.as_mut_ptr(), out.len());
                
                if result != trpak.files[i].decompressed_size as i32 {
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