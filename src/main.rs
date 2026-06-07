
use crate::cli::test_programm_controller::controller;

mod io;
mod core;
mod cli;


fn main() {

    controller();
    std::process::exit(0);
}   
