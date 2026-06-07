//! # Note
//! FIXME: The main functionality is fully implemented and operational, 
//! but the code structure requires refactoring for better idiomatic Rust

use std::path::{Path};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use zip::ZipArchive;
use zip::result::ZipError;

use crate::io::check_avaibale_files::TypeRead;


pub enum ReadError{
    InvalidFile,
    WithoutTranslatebleString,
    Io(io::Error),
    ZipArchive(ZipError)
}

impl From<io::Error> for ReadError{
    fn from(value: io::Error) -> Self {
        ReadError::Io(value)
    }
}

impl From<ZipError> for ReadError{
    fn from(value: ZipError) -> Self {
        ReadError::ZipArchive(value)
    }
}

pub fn read_data( (path,flag) : (&Path,TypeRead) ) -> Result<Vec<String>,ReadError>{

    match flag{
        TypeRead::FTBquest => read_quest(path),
        TypeRead::Modification => read_mod(path),
    }
}

fn read_quest(path: &Path) -> Result<Vec<String>,ReadError>{

   if !validate_path(path, TypeFile::File, ExtensionFile::Snbt) {
        return Err(ReadError::InvalidFile);
    }

    let readed_data = fs::read_to_string(path)?;
    
    let out_data= readed_data.lines().map(|s| s.to_string()).collect();

    Ok(out_data)
}

fn read_mod(path: &Path) -> Result<Vec<String>,ReadError>{

    if !validate_path(path, TypeFile::File, ExtensionFile::Jar){
        return Err(ReadError::InvalidFile);
    };

    let file =  File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len(){

        if let Ok(file_name) = archive.by_index(i){
            let name_path = file_name.name();
            
            if name_path.ends_with("lang/en_us.json"){
                let mut out: Vec<String> = Vec::new();
                let reader = BufReader::new(file_name);
                for i in reader.lines(){
                    out.push(i?);
                }
                return Ok(out);
            }
        }
    }

    Err(ReadError::WithoutTranslatebleString)
}

enum TypeFile {
    File,
    Dir,
}

enum ExtensionFile{
    Json,
    Jar,
    Snbt,
    None
}

impl ExtensionFile {
    fn as_str(&self) -> &str{
        match &self {
            Self::Jar => "jar",
            Self::Json => "json",
            Self::Snbt => "snbt",
            Self::None => ""
        }
    }
}

fn validate_path(target_path: &Path, type_file: TypeFile, ext_file: ExtensionFile) -> bool{

    let match_type_file = match type_file {
        TypeFile::File => target_path.is_file(),
        TypeFile::Dir => target_path.is_dir(),
    };

    let match_ext_file = match ext_file {
        ExtensionFile::None => true,
        _ => target_path.extension().map_or(false, |e| e == ext_file.as_str()),
    };

    match_ext_file && match_type_file

}

pub fn mod_has_translatable_files(file_path: &Path) -> bool{

    if !validate_path(file_path, TypeFile::File, ExtensionFile::Jar){
        return false;
    }

    let file = match File::open(file_path) {
        Ok(a) => a,
        Err(_) => return false,
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(a) => a,
        Err(_) => return false,
    };

    for i in 0..archive.len(){
        if let Ok(path) = archive.by_index(i){
            let name_path = path.name();
            
            if name_path.ends_with("lang/en_us.json"){
                return true;
            }

        }
    }

    false
}


#[cfg(test)]

mod test{
    use super::*;



}