use std::{io, path::{Path, PathBuf}};

use crate::io::reader::{ExtensionFile, TypeFile, validate_path};




pub fn controller(){


    start();

}


fn start() -> PathBuf{

    println!("Welcome!\nPls, insert a core path which ended by /minecraft");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("invalid String");
    let input = input.trim();
    let path = Path::new(&input);
    

    if validate_path(path, TypeFile::Dir, ExtensionFile::None){
        return path.to_path_buf();
    } else {
        println!("path is: \'{}\'",path.display());
    }
    
    println!("invalid path");

    start()
    
}