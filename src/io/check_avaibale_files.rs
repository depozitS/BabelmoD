use std::{fs, path::{Path, PathBuf}};

use crate::io::reader::mod_has_translatable_files;


#[derive(Clone, Copy)]
pub enum TypeRead{
    Modification,
    FTBquest,
}

pub struct PathManager{
    core_path: PathBuf,
    added_paths: Vec<(PathBuf,TypeRead)>
}

impl PathManager {

    pub fn new(core_path: &Path) -> Self{
        Self { core_path: core_path.to_path_buf(), added_paths: Vec::new() }
    }

    pub fn add_path(&mut self, added_path: &Path, type_read: TypeRead){
        self.added_paths.push((added_path.to_path_buf(),type_read));
    }

    pub fn len(&self) -> usize{
        self.added_paths.len()
    }

    pub fn get_paths(&self) -> Vec<(PathBuf, TypeRead)>{

        self.added_paths
            .iter()
            .map(|(p,t)| {
                (self.core_path.join(p),*t)
            })
            .collect()
            
    }

}


/// Searches for all files that are ready for translation.
///
/// Returns a struct PathManager of file paths that meet the translation criteria.
pub fn find_localizable_files(core_path: &Path) -> PathManager{

    let mut reader_paths:PathManager = PathManager::new(core_path);

    //quest search
    find_quest_files(core_path, &mut reader_paths);

    //mods search
    find_mods_files(core_path, &mut reader_paths);

    reader_paths
}


fn find_quest_files(core_path: &Path, reader_paths: &mut PathManager){

    let quest_lang = Path::new("config/ftbquests/quests/lang/en_us.snbt");
    if core_path.join(quest_lang).is_file(){
        reader_paths.add_path(quest_lang, TypeRead::FTBquest);
    }

}

fn find_mods_files(core_path: &Path, reader_paths: &mut PathManager){
    
    let mod_path = Path::new("mods/");
    let full_path = core_path.join(mod_path);

    if let Ok(files) = fs::read_dir(full_path){
        for file in files.flatten(){
            let file_path = file.path();
            if mod_has_translatable_files(&file_path){
                reader_paths.add_path(
                    &file_path.strip_prefix(core_path).unwrap(),
                    TypeRead::Modification
                );
            }
        }
    } 
}



