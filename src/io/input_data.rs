use std::path::Path;
use std::fs;

use crate::io::parsers::quests_parser::SnbtParseMode;

use super::parsers::quests_parser::parse_snbt; 
// mods: context clearer, more pleasant to the ear, balance
// on FTB it's clear how translation works: file -> language -> displays what's written
// on ATM10 it's not clear, there are separate folders and files
// Grok said it reads from files and writes to a common file, I checked separately and together but nothing changed
// TODO: check by setting up the build from scratch
pub struct Quest{
    pub title : String,
    pub subtitle : String,
    pub description : Vec<String>,
    pub id : u64              //u64::from_str_radix,
}


pub(crate) fn get_original_data_quests(path: &Path) -> Vec<Quest>{ //-

    let mut out = Vec::new();

    let placeholder_path = path.join("config/ftbquests/quests/lang/en_us.snbt");

    let file_data = fs::read_to_string(placeholder_path).unwrap(); //bad impletation, because retutnder result not String

    let parsed_data_by_quests = parse_snbt(SnbtParseMode::Unified, file_data);

    //placeholder
    out.push(
        Quest{
            title: "title".to_string(),
            subtitle: "subtitle".to_string(),
            description: vec![
                "description".to_string(),
                "description".to_string()
                ],
            id: 0,
        }
    );

    out
    
}