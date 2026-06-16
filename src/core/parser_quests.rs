
use crate::{error_handler::backend_error_handler::{AppError, ParserError}, models::parser_models::{ParsedData, ValuePart}};


const LETTERS: [char; 7] = ['\n', '"', '{', '}', '[', ']', '@' ]; 


pub fn parser_quests<'a>(data: &'a str) -> Result<Vec<ParsedData<'a>>,AppError>{

    let separeted_data = parse_by_states(&data);

    parser_sanitizer(separeted_data)

}


fn parser_sanitizer<'a>(data:Vec<(&'a str, &'a str)>) -> Result<Vec<ParsedData<'a>>, AppError>{

    if data.len() == 0{
        return Err(AppError::ParsedError(ParserError::DataEmpty("after parsing by states Vec empty".to_string())));
    }

    let mut out = Vec::new();

    for (data_key, data_value) in data{

        if data_key.is_empty(){
            return Err(AppError::ParsedError(ParserError::DataInvalid("key is empty".to_string())));
        }

        if data_value.is_empty() {
            return Err(AppError::ParsedError(ParserError::DataInvalid("value is empty".to_string())));
        }

        let mut parsed_value: Vec<ValuePart> = Vec::new();

        let data_for_segment: Vec<&str> = data_value
            .split('\n')
            .map(|f|{
                let after_prefix = f.strip_prefix('"').unwrap_or(f);
                after_prefix.strip_suffix('"').unwrap_or(after_prefix)
            })
            .collect();

        for line in data_for_segment{
            if !line.contains(|c| LETTERS.contains(&c)) {
                parsed_value.push(ValuePart::Translatable(line));
                parsed_value.push(ValuePart::NewLine);
            } else {
                parsed_value.extend(parse_line_by_special_symbols(&line));
                parsed_value.push(ValuePart::NewLine);
            }
        }

        out.push(ParsedData::new(data_key.trim(), parsed_value));

    }

    if out.len() < 1 { return Err(AppError::ParsedError(ParserError::LogicError("out array is empty".to_string())));}

    Ok(out)
}

fn parse_line_by_special_symbols<'a>(line: &'a str) -> Vec<ValuePart<'a>>{

    let line_new = line.strip_prefix('[').unwrap_or(line);
    let line = line_new.strip_suffix(']').unwrap_or(line_new);

    let mut out= Vec::new();

    let mut start_pos: Option<usize> = None;

    let mut is_quote_open = false;
    let mut is_shielding;
    let mut counter_backslash = 0;

    let total_chars = line.chars().count();

    for (i, ch) in line.char_indices(){

        if ch == '\\' {
            counter_backslash += 1;
        } else{
            counter_backslash = 0;
        }

        if counter_backslash % 2 != 0 {is_shielding = true}
        else {is_shielding = false}

        match (ch, is_shielding) {

            ('{', false) =>{
                if i != 0 && start_pos.is_some() && Some(i) != start_pos{
                    out.push(ValuePart::Translatable(&line[start_pos.unwrap()..i].trim()));
                }
                start_pos = Some(i);
                is_quote_open = true;
            }

            ('}' ,false) => {
                if i != 0 && start_pos.is_some() && Some(i) != start_pos && is_quote_open{
                    out.push(ValuePart::SysData(&line[start_pos.unwrap()..=i]));
                }
                start_pos = None;
                is_quote_open = false;
            }

            (_,_) => {
                if start_pos == None && !is_quote_open{
                    start_pos = Some(i);
                }
                if i == total_chars - 1 && !is_quote_open{
                    out.push(ValuePart::Translatable(&line[start_pos.unwrap()..=i].trim()));
                }
            }
        }
    }

    out
}


fn parse_by_states(data: &str) -> Vec<(&str, &str)>{
    
    let mut out: Vec<(&str, &str)> = Vec::new();

    //states
    //let mut state_curly_braket_depth: u32 = 0;
    let mut state_square_braket_open = false;
    let mut state_quote_open = false;

    let mut state_new_line = true;

    let mut index_key_start = 0;
    let mut index_key_end = 0;
    let mut index_value_start = 0;
    let mut index_value_end = 0;

    let mut is_shielding: bool;
    let mut counter_backslash = 0;

    //logic
    for (indx, ch) in data.char_indices(){
        
        if ch == '\\' {
            counter_backslash += 1;
        } else{
            counter_backslash = 0;
        }

        if counter_backslash % 2 != 0 {is_shielding = true}
        else {is_shielding = false}

        if state_new_line && !state_square_braket_open {
            index_key_start = indx;
            state_new_line = false;
        };

        match (ch, is_shielding) {

            ('"', false) => {
                state_quote_open = !state_quote_open;
            }

            (':', _) => {
                if !state_quote_open{
                    index_key_end = indx; 
                    index_value_start = indx+1;
                }
            }

            ('[', false) => {
                state_square_braket_open = true; 
                index_value_start = indx;
            }

            (']', false) => {
                state_square_braket_open = false; 
                index_value_end = indx + 1;
                if index_value_start < index_value_end && index_key_start < index_key_end && !state_quote_open{
                    out.push(get_slices(data, index_key_start, index_key_end, index_value_start, index_value_end));
                    index_key_start = 0; index_key_end = 0; index_value_start = 0; index_value_end = 0; 
                }
            }

            ('\n', _) => {
                state_new_line = true;
                if index_key_start < index_key_end && !state_square_braket_open && !state_quote_open{
                    index_value_end = indx; 
                    out.push(get_slices(data, index_key_start, index_key_end, index_value_start, index_value_end)); 
                    index_key_start = 0; index_key_end = 0; index_value_start = 0; index_value_end = 0;
                }

            }

            (_,_) => {}
        }

    }


    out
}


fn get_slices(data: &str, ik1: usize, ik2: usize, iv1: usize, iv2: usize) -> (&str, &str){

    let key: &str = &data[ik1..ik2].trim();
    let value: &str = &data[iv1..iv2].trim();

    (key, value)
}





#[cfg(test)]
mod test{

    use super::*;


    #[test]
    fn parse_test_single_line_1(){
        let data = "{\nchapter.0D990A1D1E088701.title: \"Steam Age: New Beginnings\"\n}";

        let out_data = vec![("chapter.0D990A1D1E088701.title", "\"Steam Age: New Beginnings\"")];

        assert_eq!(parse_by_states(data), out_data);
    }

    #[test]
    fn parse_test_single_line_2(){
        let data = "{\nchapter.0E81CBCD6B1D1895.chapter_subtitle: [\"And Other Spawners\"]\n}";

        let out_data = vec![("chapter.0E81CBCD6B1D1895.chapter_subtitle","[\"And Other Spawners\"]")];

        assert_eq!(parse_by_states(data), out_data);
    }

    #[test]
    fn parse_test_multi_line_1(){
        let data = "{\n
    	quest.002163B909070CF8.quest_desc: [
\"Allows you to see details about certain blocks/fluid, even through walls.\\n\"
\"{image:atm:textures/questpics/pneumaticcraft/block_tracker.png width:150 height:150 align:center}\"
	]
    }\n";

        let out_data = vec![("quest.002163B909070CF8.quest_desc","[\n\"Allows you to see details about certain blocks/fluid, even through walls.\\n\"
\"{image:atm:textures/questpics/pneumaticcraft/block_tracker.png width:150 height:150 align:center}\"\n\t]")];
        

        assert_eq!(parse_by_states(data), out_data);

    }

    #[test]
    fn parse_test_multi_line_2(){
        let data = "{\n
        	quest.00A7DEFCF07BC63A.quest_desc: [
\"The &zRed Katar&r is the ultimate weapon from &6ProjectE&r\"
\"\"
\"Set a keybind to ProjectE's Extra Function and it will deal 25 damage to everything around you with each activation.\"
	]
	quest.00A7DEFCF07BC63A.title: \"&cRed Katar\"
        \n}";

        let out_data = vec![
            ("quest.00A7DEFCF07BC63A.quest_desc", "[
\"The &zRed Katar&r is the ultimate weapon from &6ProjectE&r\"
\"\"
\"Set a keybind to ProjectE's Extra Function and it will deal 25 damage to everything around you with each activation.\"
	]"),
        ("quest.00A7DEFCF07BC63A.title", "\"&cRed Katar\"")
        ];


        assert_eq!(parse_by_states(data),out_data);
    }

    #[test]
    fn parse_line_by_special_symbols_test_1(){
        let input = vec![ValuePart::SysData("{@pagebraker}")];
        let out = parse_line_by_special_symbols("{@pagebraker}");
        assert_eq!(input, out)
    }

    #[test]
    fn parse_line_by_special_symbols_test_2(){
        let input = vec![ValuePart::Translatable("&6Apotheosis&r has a new &bWorld Tier&r system that can be accessed using \\\"CTRL + T\\\" by default.")];
        let out = parse_line_by_special_symbols("&6Apotheosis&r has a new &bWorld Tier&r system that can be accessed using \\\"CTRL + T\\\" by default.");
        assert_eq!(input,out)
    }

    #[test]
    fn parse_line_by_special_symbols_test_3(){
        let input = vec![
            ValuePart::Translatable("\\\"You start at \\\","),
            ValuePart::SysData("{\\\"text\\\": \\\"Haven\\\", \\\"color\\\": \\\"#7E7E7E\\\"}"),
            ValuePart::Translatable(", \\\" upon acquiring your first \\\",")
            ];
        let out = parse_line_by_special_symbols("[\\\"You start at \\\", {\\\"text\\\": \\\"Haven\\\", \\\"color\\\": \\\"#7E7E7E\\\"}, \\\" upon acquiring your first \\\",");
        assert_eq!(input,out);
    }

    #[test]
    fn parse_line_by_special_symbols_test_4(){
        let input = vec![
            ValuePart::SysData("{}"),
            ValuePart::SysData("{}")
        ];
        let out = parse_line_by_special_symbols("{}{}");
        assert_eq!(input, out)
    }
}
