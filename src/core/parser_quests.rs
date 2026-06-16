

pub fn parser_quests(data: String){

    todo!("data -> HashMap where key type.id.subtype and value &str ");


    


}



pub fn parse_by_states(data: &str) -> Vec<(&str, &str)>{
    
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
    //let mut iter = data.char_indices().peekable();
    
    for (indx, ch) in data.char_indices(){
        
        if ch == '\\' {
            counter_backslash += 1;
        } else{
            counter_backslash = 0;
        }

        if counter_backslash % 2 != 0 {is_shielding = true}
        else {is_shielding = false}
         

        // if ch == '{' {state_curly_braket_depth += 1; continue;}
        // if ch == '}' {state_curly_braket_depth -= 1; continue;}

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
}
