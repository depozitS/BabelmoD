mod error_handler;
mod io;
mod models;
mod core;
mod api;

use std::path::Path;
use crate::{core::parser_quests::parser_quests, io::readers::read_regular_file};

fn main() {

    let pt = Path::new("/home/Yk4m4/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/FTB StoneBlock 4/minecraft/config/ftbquests/quests/lang/en_us.snbt");
    let a = read_regular_file(pt);
    let b = match a{
        Ok(a) => a,
        Err(a) => String::new()
    };
    let c = parser_quests(&b);

    let d = match c{
        Ok(c) => c,
        _ => Vec::new()
    };

    let  mut counter_lines: usize = 0;

    for i in d{
        counter_lines += i.value.len() + 1;
    }

    println!("{counter_lines}")

}   


