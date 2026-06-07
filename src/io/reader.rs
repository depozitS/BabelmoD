
use std::path::{Path, PathBuf};
use std::fs::{read_to_string, File};
use std::io;
use zip::ZipArchive;

use crate::io::check_avaibale_files::TypeRead;

pub fn read_data(core_path: &Path, added_path: &Path, type_read_flag: TypeRead) -> io::Result<Vec<String>>{

    match type_read_flag{

        
        TypeRead::FTBquest =>{

            //add validate path and sepatet read logic in self fn

            let mut out = Vec::new();


            //TODO: add safe join
            let full_path = core_path.join(added_path);

            let readed_data = read_to_string(full_path)?;

            for line in readed_data.lines(){
                out.push(line.to_string());
            }

            Ok(out)
        },

        //placeholder
        _ => todo!("Implement reading for mod variant"),
    }
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