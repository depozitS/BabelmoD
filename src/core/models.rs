

pub enum FieldType{
    Title,
    QuestSubtitle,
    ChapterSubtitle,
    Description,
}

impl FieldType {
    pub fn get_field(data: &str) -> Self{
        match data {
            "quest_subtitle" => FieldType::QuestSubtitle,
            "chapter_subtitle" => FieldType::ChapterSubtitle,
            "quest_desc" => FieldType::Description,
            "title" => FieldType::Title,
            _ => {
                eprintln!("[WARN] Unknown field '{}', mapping to title", data);
                FieldType::Title
                }
        }

    }
}
#[derive(Debug, PartialEq)]
pub enum TranslatableFlag{
    Translable,
    Skip,
}

/// DON'T TRY TO MODIFY THE STRUCTURES MANUALLY
/// CORRECT WORK IS GUARANTEED ONLY BY USING `.new()` AND `.update()`
#[derive(Debug,PartialEq)]
pub struct QuestData{

    pub id: u64,
    pub group: String,
    
    pub title: Vec<(String, TranslatableFlag)>,
    pub quest_subtitle: Vec<(String, TranslatableFlag)>,
    pub chapter_subtitle: Vec<(String, TranslatableFlag)>,
    pub description: Vec<(String, TranslatableFlag)>,
}


impl QuestData{

    pub fn new(id:u64, group: String, field: FieldType, data: Vec<String>) -> QuestData{

        let mut out = QuestData { 
            id,
            group, 
            title: Vec::new(), 
            quest_subtitle: Vec::new(), 
            chapter_subtitle: Vec::new(),
            description: Vec::new(), 
        };

        out.update(field, data);

        out

    }

    /// Since the data comes from the parser divided by FieldType, to add a structure with the same ID,
    /// the `update` method is used.
    pub fn update(&mut self, field: FieldType, data: Vec<String>) {
        match field {
            FieldType::Title => {self.title = Self::validate_data(data)},
            FieldType::QuestSubtitle => {self.quest_subtitle = Self::validate_data(data)},
            FieldType::ChapterSubtitle => {self.chapter_subtitle = Self::validate_data(data)},
            FieldType::Description => {self.description = Self::validate_data(data)},
        }

    }


    /// Initially written for verifying the received data
    /// Used for verifying the initial file and the parsed file by line count
    pub fn lines_count(&self) -> usize {self.title.len() + self.quest_subtitle.len() + self.description.len() + self.chapter_subtitle.len()}

    /// Internal function for marking lines as translatable
    fn validate_data(input: Vec<String>) -> Vec<(String,TranslatableFlag)> {
        //todo!("realize validate_data");

        let mut out = Vec::new();

        for i in input{
            out.push((i, TranslatableFlag::Translable));
        }

        out
    }

}