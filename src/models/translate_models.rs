use std::{ops::Index, path::PathBuf};

use crate::error_handler::backend_error_handler::{AppError, ParserError};

#[derive(Clone,Copy)]
pub struct TextSlice{
    pub start: usize,
    pub end: usize
}

pub enum ValuePart{
    Translatable(TextSlice),
    SysData(TextSlice),
    NewLine
}

pub struct ParsedValue {
    pub key: TextSlice,
    pub value: Vec<ValuePart>
}


pub struct ProjectPart{
    pub file_path: PathBuf,
    pub file_path_in_archive: Option<PathBuf>,

    pub original_data: String,
    pub parsed_data: Vec<ParsedValue>,

    pub translatable_data: String,
    pub translatable_slice: Vec<ParsedValue>,
}

pub struct Project{
    project_name: String,
    part: Vec<ProjectPart>
}



impl TextSlice {
    pub fn new(start: usize, end: usize) -> Result<Self, AppError>{

        if end < start{
            return Err(AppError::ParsedError(ParserError::DataInvalid("Invalid slice bounds: start cannot be greater than end".to_string())));
        }

        Ok(Self { start, end })
    }
}

impl Index<TextSlice> for str{
    type Output = str;
    fn index(&self, index: TextSlice) -> &Self::Output {
        &self[index.start..index.end]
    }
}

impl Index<TextSlice> for String{
    type Output = str;
    fn index(&self, index: TextSlice) -> &Self::Output {
        &self[index.start..index.end]
    }
}

impl ParsedValue{
    pub fn new(key: TextSlice) -> Self{
        Self { key, value: Vec::new() }
    }

    pub fn add_value(&mut self, value: ValuePart){
        self.value.push(value);
    }
}

impl ProjectPart {
    pub fn new(path: PathBuf, path_in_archive: Option<PathBuf>, orig_data: String, orig_slices: Vec<ParsedValue>) -> Result<Self, AppError>{

        let mut out = Self { 
            file_path: path, 
            file_path_in_archive: path_in_archive, 
            original_data: orig_data, 
            parsed_data: orig_slices, 
            translatable_data: String::new(), 
            translatable_slice: Vec::new(), 
        };

        out.add_keys_to_translatable()?;

        Ok(out)

    }


    fn add_keys_to_translatable(&mut self) -> Result<(), AppError>{
        for i in &self.parsed_data{

            if i.key.end > self.original_data.len(){
                return Err(AppError::ParsedError(ParserError::LogicError("slice out of bounds".to_string())));
            }

            let slice = &self.original_data[i.key];
            
            if !slice.contains('.') ||
                slice.contains('"') || 
                slice.contains('\n')
            {
                return Err(AppError::ParsedError(ParserError::LogicError(
                    format!(
                        "after parsing data have wrong format: {}", slice
                    )
                )));
            }

            let start = self.translatable_data.len();
            self.translatable_data.push_str(slice);
            let end = self.translatable_data.len();

            self.translatable_data.push('\n');
            self.translatable_slice.push(
                ParsedValue { 
                    key: TextSlice { start, end }, 
                    value: Vec::new() }
            );
        }

        Ok(())

    }
    
}