<<<<<<< HEAD
mod error_handler;
mod io;
mod models;
mod core;

use std::path::Path;
use crate::io::readers::read_original_data;
use crate::error_handler::backend_error_handler::AppError;
use crate::models::io_models::{self, FlagRead};

fn main() {

=======

use crate::cli::test_programm_controller::controller;

mod io;
mod core;
mod cli;


fn main() {

    controller();
    std::process::exit(0);
>>>>>>> main
}   
