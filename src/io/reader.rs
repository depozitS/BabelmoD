
use std::path::{Path, PathBuf};
use std::fs::read_to_string;
use std::io;

use crate::io::check_avaibale_files::TypeRead;





pub fn read_data(core_path: &Path, added_path: &Path, type_read_flag: TypeRead) -> io::Result<Vec<String>>{

    match type_read_flag{

        
        TypeRead::FTBquest =>{
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




#[cfg(test)]

mod test{
    use super::*;



}