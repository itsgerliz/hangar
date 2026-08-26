use std::path::PathBuf;

pub struct HangarState {
    database: PathBuf,
    data_directory: PathBuf,
}

impl HangarState {
    pub fn new<P: Into<PathBuf>>(database: P, data_directory: P) -> Self {
        Self {
            database: database.into(),
            data_directory: data_directory.into(),
        }
    }
}
