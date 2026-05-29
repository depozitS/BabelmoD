use std::path::{Path, PathBuf};

pub enum GameVersion{
    V1_21_1
}
struct TableLocalizableFiles{
    quests: Vec<(String, PathBuf)>,      //names of locazible ftbquests files
    //TODO: add another typed  e.g. mods or kube js
}

pub fn get_localizable_files_stats(core_path: &Path, version: Option<GameVersion> ) -> TableLocalizableFiles{

    let version = version.unwrap_or(GameVersion::V1_21_1);

    match version{
        
        GameVersion::V1_21_1 =>{

            let path_quest = core_path.join("/config/ftbquests/quests/lang");

            if path_quest.is_dir() {

            }

        }

    }



    todo!("add methods to read core directory of modpack")
}

