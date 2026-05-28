





#[derive(PartialEq)]
pub enum SnbtParseMode{
    Unified,
    Split,
}

#[derive(PartialEq, Clone, Copy)]
pub enum GroupType {
    Chapter,
    ChapterGroup,
    Quest,
    Task,
    Reward,
    RewardTable,
    None
}

#[derive(PartialEq)]
pub enum DataType {
    Title,
    Subtitle,
    Description,
    None,
}

// removes quotes or brackets and quotes, follows a strict rule from .snbt files in FTB Quest
// compatibility with other versions of the FTB Quest mod is not guaranteed
fn trim_quotes(input: &str) -> &str{ 

    let trimmed = input.trim();

    if let Some(i) = trimmed.strip_prefix("[\"").and_then(|trimmed| trimmed.strip_suffix("\"]")){
        return i;
    }

    if let Some(i) = trimmed.strip_prefix("\"").and_then(|trimmed| trimmed.strip_suffix("\"")){
        return i;
    }

    trimmed

}

// Converts the ID from hexadecimal to u64 for convenient use within the program
fn id_converter(input: &str) -> u64{
    u64::from_str_radix(input,16).unwrap()
}


// Parses the string in parts for convenient assembly in the future
// SnbtParseMode::Split is in development; examples of why this is needed exist in ATM10
pub fn parse_snbt(mode: SnbtParseMode, data: String) -> Vec<(GroupType, u64, DataType, String)>{

    let mut out = Vec::new();

    match mode{

        //placeholder
        SnbtParseMode::Split => {

            out.push((
                GroupType::Quest, 0, DataType::Title, ("title".to_string())
            )); //output tuple: type text, id, data 

        }

        // Currently the main method of operation
        SnbtParseMode::Unified => {

            // Iterator over lines, .skip(1) is used because the first line in .snbt is "{"
            let mut lines = data.lines().skip(1);

            while let Some(line) = lines.next(){

                let mut separated_line = line.splitn(3,".");

                let (prefix_type,id_hex_string, (title_inbound, data_inbound)) = (
                    separated_line.next().unwrap_or("").trim(),
                    separated_line.next().unwrap_or("").trim(),
                    separated_line.next().unwrap_or("").split_once(":").unwrap_or(("", "")),
                    );
                
                let group_converted = match prefix_type{
                    "chapter_group" => GroupType::ChapterGroup,
                    "chapter" => GroupType::Chapter,
                    "quest" => GroupType::Quest,
                    "task" => GroupType::Task,
                    "reward_table" => GroupType::RewardTable,
                    "reward" => GroupType::Reward,  
                    _ => GroupType::None,
                };

                let type_converted = match title_inbound{
                    "title" => DataType::Title,
                    "quest_subtitle" => DataType::Subtitle,
                    "quest_desc" => DataType::Description,
                    _ => DataType::None,
                    
                };

                let mut desc: String;

                // Description can be written in two formats: single-line and multi-line
                // To prevent the loop from running indefinitely when encountering a multi-line structure
                // The algorithm branches based on the situation (single-line vs multi-line)
                if type_converted == DataType::Description {

                    if !data_inbound.contains("]"){

                        desc = String::new();

                        while let Some(buf_line) = lines.next() {

                            if buf_line.trim().len() > 1{

                                desc.push_str(trim_quotes(buf_line));
                                desc.push('\n'); // This is necessary to prevent multi-line structures from being merged into a single line

                            }

                            
                            if buf_line.contains("]") && !buf_line.contains("\"]\""){
                                break;
                            }

                        }

                    }
                    else {

                        desc = trim_quotes(data_inbound).to_string();

                    }

                } else {

                    desc = trim_quotes(data_inbound).to_string();

                }
                
                //store data in (group, id, type, text)
                out.push((
                    group_converted,
                    id_converter(id_hex_string),
                    type_converted,
                    desc,

                ));
                

            };           
            
        }

    }

    out

}