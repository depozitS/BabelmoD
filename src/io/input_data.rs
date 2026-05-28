use std::collections::HashMap;
use std::{path::Path};
use std::fs;
use std::io;

use crate::io::parsers::quests_parser::DataType;
use crate::io::{ parsers::quests_parser::{GroupType, SnbtParseMode}};

use super::parsers::quests_parser::parse_snbt; 
// mods: context clearer, more pleasant to the ear, balance
// on FTB it's clear how translation works: file -> language -> displays what's written
// on ATM10 it's not clear, there are separate folders and files
// Grok said it reads from files and writes to a common file, I checked separately and together but nothing changed
// TODO: check by setting up the build from scratch

pub enum TranslationFlag {
    Translatable,  
    Skip,   
}

pub struct TaskStruct{
    pub group : GroupType,
    pub id : u64, 
    pub title : Option<String>,
    pub subtitle : Option<String>,
    pub description : Vec<(String, TranslationFlag)>,
}


impl TaskStruct {

    pub fn create((group, id, type_data, data):(GroupType, u64, DataType, String)) -> TaskStruct{

        let mut title_string = None;
        let mut subtitle_string = None;
        let mut desc_string: Vec<(String, TranslationFlag)> = Vec::new();
        match type_data{
            DataType::Title => title_string = Some(data),
            DataType::Subtitle => subtitle_string = Some(data),
            DataType::Description => desc_string = Self::unpack_description(data),
            _ => panic!("panic in id{}",id),
        }

        TaskStruct { 
            group,
            id,
            title: title_string,
            subtitle: subtitle_string,
            description: desc_string, 
        }
    } //кортеж (a,b):(atype, btype)

    pub fn update(&mut self, (_group, id, type_data, data):(GroupType, u64, DataType, String)){
        match type_data{
            DataType::Title => self.title = Some(data),
            DataType::Subtitle => self.subtitle = Some(data),
            DataType::Description => self.description = Self::unpack_description(data),
            _ => panic!("panic in id{}",id),
        }
    }

    fn unpack_description(data: String) -> Vec<(String,TranslationFlag)>{
        let mut out: Vec<(String,TranslationFlag)> = Vec::new();

        for line in data.lines(){

            let flag = if line.contains("{") || line.trim().is_empty() {TranslationFlag::Skip} else {TranslationFlag::Translatable};

            out.push((line.to_string(),flag));
        }

        out

    }
    
}


pub(crate) fn get_data_quests(path: &Path) -> io::Result<Vec<TaskStruct>>{ 

    let file_data = fs::read_to_string(path)?;


    //let mut parsed_data_iter = parsed_data_by_quests.iter();
    let parsed_data_by_quests = parse_snbt(SnbtParseMode::Unified, file_data);

    let mut table_quest: HashMap<u64,TaskStruct> = HashMap::new();

    for parsed_tuple in parsed_data_by_quests.into_iter(){

        if let Some(task) = table_quest.get_mut(&parsed_tuple.1) {
            task.update(parsed_tuple);

        } else {
   
            table_quest.insert(parsed_tuple.1, TaskStruct::create(parsed_tuple));

        }

    }

    Ok(table_quest.into_values().collect())
    
}