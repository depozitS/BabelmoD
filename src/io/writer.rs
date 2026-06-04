use std::{fs, path::Path};

use crate::core::models::QuestData;



pub fn quest_writer(core_path: &Path, added_path: &Path, data: Vec<QuestData>) /*->Reslut*/{

    let out_path = core_path.join(added_path);

    let mut builded_out_string = String::new();
    builded_out_string.push_str("{\n");

    for i in data{
        builded_out_string.push_str(i.assemble().trim());
        builded_out_string.push_str("\n");
    }

    builded_out_string.push_str("}\n");

    fs::write(out_path, builded_out_string);

}