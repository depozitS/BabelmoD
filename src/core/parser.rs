

use std::{iter, num::ParseIntError};

use indexmap::IndexMap;

use crate::core::{models::{FieldType, QuestData}, parser::ParserError::ParserMistake};


pub enum ParserError{
    InvalidFormat(String),
    ParserMistake(String)
}

impl From<ParseIntError> for ParserError {
    fn from(err: ParseIntError) -> Self {
        ParserError::ParserMistake(format!("failed to parse hex ID: {}", err))
    }
}

fn validate_data(data: &[String]) -> Result<(),ParserError>{

    if data.is_empty(){
        return Err(ParserError::InvalidFormat("file is empty!".to_string()));
    }

    let (first_line, last_line) = (&data[0], &data[data.len()-1]);

    if !first_line.trim().starts_with("{") || !last_line.trim().ends_with("}"){
        return Err(ParserError::InvalidFormat("This is a file that doesn't contain any curly braces { or }. I have no idea what kind of file it is.".to_string()));
    }

    Ok(())

}

fn convert_hex_id(data: &str) ->Result<u64, std::num::ParseIntError>{
    u64::from_str_radix(data,16)
}



pub fn parse_data_quest(data :Vec<String>) -> Result<IndexMap<u64,QuestData>,ParserError>{
    
    let mut out: IndexMap<u64,QuestData> = IndexMap::new();
    validate_data(&data)?;

    let mut group;
    let mut id: u64;
    let mut field: FieldType;
    let mut data_compose: Vec<String> = Vec::new();
    let mut depth = 0;
    
    for line in &data{

        if depth == 0{

            data_compose.clear();

            if let Some((data_sys, data_str)) = line.split_once(":"){
                

                let mut parts = data_sys.splitn(3, ".");

                if let (Some(group_str), Some(id_str), Some(field_str)) = (parts.next(),parts.next(),parts.next()){
                    group = group_str.to_string();
                    id = convert_hex_id(id_str)?;
                    field = FieldType::get_field(field_str);
                } else {
                    return Err(ParserMistake(format!("didnt found delimeter . in {}",line)));
                }

                depth = data_str.matches('[').count() - data_str.matches(']').count();
                data_compose.push(data_str.trim().to_string());

                //логическая дыра, при многострочном вводе
                if depth == 0{

                    if let Some(quest) = out.get_mut(&id) {
                        quest.update(field, std::mem::take(&mut data_compose));
                    } else {
                        out.insert(id, QuestData::new(id, group.clone(), field, std::mem::take(&mut data_compose)));
                    }

                }

            } else { 
                return Err(ParserError::ParserMistake(format!("didnt found delimeter : in {}",line)));
            };

        } else {

            data_compose.push(line.to_string());
            depth += line.matches('[').count() - line.matches(']').count();
            
        }

    }

    Ok(out)

}