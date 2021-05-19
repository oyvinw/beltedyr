use std::{fs::File, path::Path};

mod ldtk_0_9_3;

pub impl Project {

    pub fn load(path: String) -> Self {
        let file_path = Path::new(&f);
        let file = File::open(file_path).expect("LDtk files not found");
        let o Project = serde_json::from_reader(file).expect("Error parsing JSON");
    }

}