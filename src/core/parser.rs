
use std::{num::ParseIntError};

use indexmap::IndexMap;

use crate::core::{models::{FieldType, QuestData}, parser::ParserError::ParserMistake};

#[derive(Debug, PartialEq)]
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






//Tests
#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn test_parser_1(){

        let data = vec!["{".to_string(),"data.FF.title: \"hello, world\"".to_string(),"}".to_string()];
        let out_data_struct = QuestData::new(
            255, 
            "data".to_string(), 
            FieldType::Title, 
            vec!["\"hello, world\"".to_string()]
        );

        let mut out_data: IndexMap<u64,QuestData> = IndexMap::new();
        out_data.insert(255, out_data_struct);

        assert_eq!(
            parse_data_quest(data),
            Ok(out_data)
        )
    }

    #[test]
    fn test_parser_2(){

        let data = vec![
        "{".to_string(),
        "finish.A0F2.quest_desc: [".to_string(),
        "\"hello blyat\"".to_string(),
        "{@pagebraker}".to_string(),
        "\"bye bye\"".to_string(),
        "]".to_string(),
        "}".to_string(),
        ];

        let expected_lines = vec![
            "[".to_string(),
            "\"hello blyat\"".to_string(),
            "{@pagebraker}".to_string(),
            "\"bye bye\"".to_string(),
            "]".to_string(),
    ];

    let out_data_struct = QuestData::new(
        41202, 
        "finish".to_string(), 
        FieldType::Description,
        expected_lines
    );

    let mut out_data: IndexMap<u64, QuestData> = IndexMap::new();
    out_data.insert(41202, out_data_struct);

    assert_eq!(
        parse_data_quest(data),
        Ok(out_data)
    );

    }

    #[test]
    fn test_parser_3(){
        let data = vec![
        "{".to_string(),
        "finish.FF032.title: \"its a title bro\"".to_string(),
        "finish.FF032.quest_desc: [".to_string(),
        "\"hello blyat\"".to_string(),
        "{@pagebraker}".to_string(),
        "\"bye bye\"".to_string(),
        "]".to_string(),
        "}".to_string(),
        ];


        let mut out_data_struct = QuestData::new(
            1044530, 
            "finish".to_string(), 
            FieldType::Title, 
            vec!["\"its a title bro\"".to_string()],
        );
    
        out_data_struct.update(
            FieldType::Description,
            vec![
                "[".to_string(),
                "\"hello blyat\"".to_string(),
                "{@pagebraker}".to_string(),
                "\"bye bye\"".to_string(),
                "]".to_string(),
                ]
            );
        
        let mut out_data: IndexMap<u64, QuestData> = IndexMap::new();
        out_data.insert(1044530, out_data_struct);

        assert_eq!(
            parse_data_quest(data),
            Ok(out_data),
        );
    }

    #[test]
    fn parser_test_empty_input(){
        assert_eq!(
            parse_data_quest(vec![]),
            Err(ParserError::InvalidFormat("file is empty!".to_string()))
        )
    }

    #[test]
    fn parser_test_invalid_format(){
    
        let data_wrong = vec!["dataFF.title: \"hello, world\"".to_string(),"}".to_string()];
        let data2_wrong = vec!["{".to_string(), "dataFF.title: \"hello, world\"".to_string()];
        let data3_wrong= vec!["[".to_string(), "dataFF.title: \"hello, world\"".to_string(),"}".to_string()];

        assert_eq!(parse_data_quest(data3_wrong),parse_data_quest(data2_wrong.clone()));
        assert_eq!(parse_data_quest(data_wrong),parse_data_quest(data2_wrong.clone()));

        assert_eq!(
            parse_data_quest(data2_wrong),
            Err(ParserError::InvalidFormat("This is a file that doesn't contain any curly braces { or }. I have no idea what kind of file it is.".to_string()))
        )
    }

    #[test]
    fn parser_test_didnt_found_delimeter(){
        let data1_wrong= vec!["{".to_string(), "dataFF.title: \"hello, world\"".to_string(),"}".to_string()];
        let data2_wrong= vec!["{".to_string(), "data.FF.title \"hello, world\"".to_string(),"}".to_string()];

        assert_eq!(
            parse_data_quest(data1_wrong),
            Err(ParserError::ParserMistake("didnt found delimeter . in dataFF.title: \"hello, world\"".to_string()))
        );

        assert_eq!(
            parse_data_quest(data2_wrong),
            Err(ParserError::ParserMistake("didnt found delimeter : in data.FF.title \"hello, world\"".to_string()))
        );
    }

    #[test]
    fn parser_test_invalid_hex(){

        let data = vec!["{".to_string(),"data.FX.title: \"hello, world\"".to_string(),"}".to_string()];

        assert_eq!(
            parse_data_quest(data),
            Err(ParserError::ParserMistake("failed to parse hex ID: invalid digit found in string".to_string())),
        )
    }

    #[test]
    fn parser_test_array_followed_by_field() {
        let data = vec![
            "{".to_string(),
            "finish.A0F2.quest_desc: [".to_string(),
            "\"line 1\"".to_string(),
            "]".to_string(),
            "finish.A0F2.title: \"back to normal\"".to_string(),
            "}".to_string(),
        ];

        let mut expected_quest = QuestData::new(
            41202,
            "finish".to_string(),
            FieldType::Description,
            vec!["[".to_string(), "\"line 1\"".to_string(), "]".to_string()],
        );
        expected_quest.update(FieldType::Title, vec!["\"back to normal\"".to_string()]);

        let mut expected_map = IndexMap::new();
        expected_map.insert(41202, expected_quest);

        assert_eq!(parse_data_quest(data), Ok(expected_map));
    }

}