use std::path::Path;

use crate::io::reader::{TypeRead, read_data};

mod io;
mod core;


fn main() {


    let core_path = Path::new("/home/Yk4m4/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/FTB StoneBlock 4/minecraft/");
    let added_path = Path::new("config/ftbquests/quests/lang/en_us.snbt");

    let mut a = read_data(core_path, added_path, TypeRead::FTBquest);

    println!("{}",a.expect("oops").len());

}
