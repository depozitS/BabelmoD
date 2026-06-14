
pub enum FlagRead{
    FTBQuest,
    MinecraftMod,
}



#[derive(PartialEq)]
pub enum ExtensionFile {
    SNBT,
    JAR
}

#[derive(PartialEq)]
pub enum TypeFile {
    DIR,
    FILE
}

impl ExtensionFile {
    pub fn to_str(&self) -> &str{
        match &self {
            ExtensionFile::SNBT => "snbt",
            ExtensionFile::JAR => "jar"
        }
    }
}

impl TypeFile {
    pub fn to_str(&self) -> &str{
        match &self {
            TypeFile::DIR => "dir",
            TypeFile::FILE => "file"
        }
    }
}