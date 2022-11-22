use std::path::Path;
use std::io::{Write};
use std::process::Command;
use std::fs::{File, create_dir_all, read_to_string};

use crate::cli::{State, TRPAK, Compression};
use crate::error_handler::SVExtractorError;


pub fn extract(state: &mut State, file_str: &String) -> Result<(), SVExtractorError> {
    extract_trpak_flatc(state, &file_str)?;
    write_files(state, &file_str)?;
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

fn write_files(_state: &State, file_str: &String) -> Result<(), SVExtractorError> {
    let json_path = Path::new(&file_str).with_extension("json");
    let trpak_str = read_to_string(&json_path)?;
    let trpak: TRPAK = serde_json::from_str(&trpak_str)?;

    for i in 0..trpak.files.len() {
        let new_file = json_path.with_extension("").join(trpak.hashes[i].to_string());
        let data = trpak.files[i].data.clone();
        
        if trpak.files[i].compression_type == Compression::OODLE {
            break;
            //oodle_decompress(&state, &data, &data.len(), &trpak.files[i].decompressed_size);
        }

        if !Path::new(&new_file).exists() {
            create_dir_all(Path::new(&new_file).parent().unwrap())?;
        }

        let mut out_file = File::create(new_file)?;
        out_file.write_all(&data)?;
    }

    Ok(())
}

fn oodle_decompress(_state: &State, _raw_bytes: &Vec<u8>, _size: &usize, _output_size: &u64) {
    let _string_buffer: String;
    todo!();
}

