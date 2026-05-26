use std::path::Path;

use crate::io::input_data::get_original_data_quests;


mod io;


fn main() {
    let placeholder_link = Path::new("/home/Yk4m4/.var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances/FTB StoneBlock 4/minecraft/");

    get_original_data_quests(placeholder_link);
}
