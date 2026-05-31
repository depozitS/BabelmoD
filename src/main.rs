use std::path::Path;

use crate::{core::{models::QuestData, parser::parse_data_quest}, io::reader::{TypeRead, read_data}};

mod io;
mod core;


fn main() {


    let core_path = Path::new("/home/Yk4m4/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/All the Mods 10 - ATM10/minecraft/");
    let added_path = Path::new("config/ftbquests/quests/lang/en_us.snbt");

    let a = read_data(core_path, added_path, TypeRead::FTBquest);
    let d = read_data(core_path, added_path, TypeRead::FTBquest);
    let b = parse_data_quest(a.expect("msg"));
    let c: Vec<QuestData> = b.unwrap().into_values().collect();

    let mut counter = 0;
    for i in c{
        counter+=i.lines_count();
    }


    println!("success parsed lines: {}, original lines : {}",counter, d.unwrap().len()-2);
    

}
