use basic_contact_manager::config::*;

fn main() {
    basic_contact_manager::run(
        Config {
            data_path: DATA_PATH,
            temp_path: TEMP_PATH,
            total_num_cols: TOTAL_NUM_COLS,
            id_idx: ID_IDX,
            name_idx: NAME_IDX,
            email_idx: EMAIL_IDX,
        }
    );
}
