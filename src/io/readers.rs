use std::fs;
use std::path::Path;

use crate::error_handler::backend_error_handler::{AppError, IOError, PathError, ReadError};


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

