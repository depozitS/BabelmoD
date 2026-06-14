use std::fs;
use std::path::Path;
use std::string::ParseError;

use crate::error_handler::backend_error_handler::{AppError, IOError, PathError, ReadError};
use crate::models::io_models::{ExtensionFile, FlagRead, TypeFile};



pub fn read_original_data ( (path, flag) : (&Path, FlagRead) ) -> Result<String, AppError>{

    match flag {
        FlagRead::FTBQuest =>{
            validate_path(path, TypeFile::FILE, ExtensionFile::SNBT)?;
            read_regular_file(path)
        }
        FlagRead::MinecraftMod =>{
            validate_path(path, TypeFile::FILE, ExtensionFile::JAR)?;
            read_mod(path)
        }
    }

}

fn read_regular_file(path: &Path) -> Result<String,AppError>{
    fs::read_to_string(path)
        .map_err(|e| 
            AppError::IOError(
                IOError::ReadError(
                    ReadError::FileCantRead(
                        format!("err: {}", e)
                ))))
}

fn read_mod(path: &Path) -> Result<String,AppError>{
    todo!("realize finding and reading original lang grom mod and realize AppError for mod for example ModError::ModWithoutText")
}

fn validate_path(path: &Path, type_path: TypeFile, extension_file: ExtensionFile) -> Result<(), AppError>{

    
    if !path.exists(){
        return Err(AppError::IOError(IOError::PathError(PathError::PathNotExist(path.to_string_lossy().to_string()))));
    }

    if !( (path.is_file() && type_path == TypeFile::FILE) || (path.is_dir() && type_path == TypeFile::DIR) ){
        return Err(AppError::IOError(IOError::PathError(PathError::PathMissmatch(
            format!("type: {}",type_path.to_str())
        ))));
    }

    if !path.extension().map_or(false, |e| e.to_string_lossy() == extension_file.to_str()){
        return Err(AppError::IOError(IOError::PathError(PathError::PathMissmatch(
            format!("ext: {}", extension_file.to_str())
        ))));
    }

    Ok(())
}