use std::{fs, path::{Path, PathBuf}};


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

    reader_paths
}


fn find_quest_files(core_path: &Path, reader_paths: &mut PathManager){

    let quest_lang = Path::new("config/ftbquests/quests/lang/en_us.snbt");
    if core_path.join(quest_lang).is_file(){
        reader_paths.add_path(quest_lang, TypeRead::FTBquest);
    }

}

fn find_mods_files(){
    todo!("iter mods")
}

fn mod_has_translatable_files(){
    todo!("iter files into mod to find en_us.json")
}