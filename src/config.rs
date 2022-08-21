pub struct Config {
    /// path to the csv file
    pub data_path: &'static str,
    /// path to the temp file for editing operations
    pub temp_path: &'static str,
    /// total number of columns in data
    pub total_num_cols: usize,
    /// the col index of id
    pub id_idx: usize,
    /// the col index of name
    pub name_idx: usize,
    /// the col index of email
    pub email_idx: usize,
}

pub const DATA_PATH: &str = "contact-data.csv";
pub const TEMP_PATH: &str = "contact-data.temp";
pub const TOTAL_NUM_COLS: usize = 3;
pub const ID_IDX: usize = 0;
pub const NAME_IDX: usize = 1;
pub const EMAIL_IDX: usize = 2;
