use std::path::Path;

use crate::io::input_data::get_data_quests;


mod io;


fn main() {
    let placeholder_link = Path::new("/home/Yk4m4/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/FTB StoneBlock 4/minecraft/");

    let placeholder_path = placeholder_link.join("config/ftbquests/quests/lang/en_us.snbt");

    get_data_quests(&placeholder_path);
}
