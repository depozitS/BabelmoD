





#[derive(PartialEq)]
pub enum SnbtParseMode{
    Unified,
    Split,
}

#[derive(PartialEq)]
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

fn id_converter(input: &str) -> u64{
    u64::from_str_radix(input,16).unwrap()
}



pub fn parse_snbt(mode: SnbtParseMode, data: String) -> Vec<(GroupType, u64, DataType, String)>{

    let mut out = Vec::new();

    match mode{

        //placeholder
        SnbtParseMode::Split => {

            out.push((
                GroupType::Quest, 0, DataType::Title, ("title".to_string())
            )); //output tuple: type text, id, data 

        }

        SnbtParseMode::Unified => {

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

                if type_converted == DataType::Description {

                    if !data_inbound.contains("]"){

                        desc = String::new();

                        while let Some(buf_line) = lines.next() {

                            if buf_line.trim().len() > 1{

                                desc.push_str(trim_quotes(buf_line));
                                desc.push('\n');

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

                
                println!("{} - {}",id_hex_string,desc);
                
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