
use std::{num::ParseIntError};

use indexmap::IndexMap;

use crate::core::{models::{FieldType, QuestData}, parser::ParserError::ParserMistake};

#[derive(Debug)]
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

    let mut group: Option<String> = None;
    let mut id: u64 = 0;
    let mut field: Option<FieldType> = None;
    let mut data_compose: Vec<String> = Vec::new();
    let mut depth: i32 = 0;
    
    for line in &data{

        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "{" || trimmed == "}"{
            continue;
        }

        if depth == 0{

            if let Some((data_sys, data_str)) = line.split_once(":"){
                
                data_compose.clear();

                let mut parts = data_sys.splitn(3, ".");

                if let (Some(group_str), Some(id_str), Some(field_str)) = (parts.next(),parts.next(),parts.next()){
                    group = Some(group_str.to_string());
                    id = convert_hex_id(id_str)?;
                    field = Some(FieldType::get_field(field_str));
                } else {
                    return Err(ParserMistake(format!("didnt found delimeter . in {}",line)));
                }

                depth = data_str.matches('[').count() as i32 - data_str.matches(']').count() as i32;
                data_compose.push(data_str.trim().to_string());

            } else { 
                return Err(ParserError::ParserMistake(format!("didnt found delimeter : in {}",line)));
            };

        } else {

            data_compose.push(line.to_string());
            depth += line.matches('[').count() as i32 - line.matches(']').count() as i32;
            
        }

        if group.is_some() && depth == 0 {

            if let Some(quest) = out.get_mut(&id) {
                    quest.update(field.take().unwrap(), std::mem::take(&mut data_compose));
                } else {
                    out.insert(
                        id,
                        QuestData::new(id, group.take().expect("code broken in .insert()"), 
                        field.take().unwrap(), 
                        std::mem::take(&mut data_compose))
                    );
                }
        }   

    }

    Ok(out)

}