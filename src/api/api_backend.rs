use std::{fs, path::Path};

use crate::error_handler::backend_error_handler::AppError;

pub struct DataBase{
    pub pool: sqlx::SqlitePool,
}

impl DataBase{

}