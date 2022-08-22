#[cfg(test)]
mod tests {
    use crate::{append_entry_to_data, display_table, ProgramError};

    const TEST_DATA_PATH: &str = "./__test__/data.csv";
    const TEST_TEMP_PATH: &str = "./__test__/temp.csv";
    const NON_EXISTING_PATH: &str = "./__test__/non_existing_path.csv";

    #[test]
    fn test_display_table() {
        let res = display_table(&vec!["name", "age"], &[
            vec!["Tom", "10"],
            vec!["Jerry", "15"],
            vec!["Joe", "20"],
        ]).unwrap();

        assert_eq!((), res);
    }

    #[test]
    fn test_append_entry_to_data() {
        // setup - create file at TEST_DATA_PATH
        let mut file = std::fs::File::create(TEST_DATA_PATH).unwrap();

        let mut data = &[
            vec!["Tom", "10"],
            vec!["Jerry", "15"],
            vec!["Joe", "20"],
        ];
        let entry = ("Alice", "30");
        let err = append_entry_to_data(entry, data, TEST_DATA_PATH).unwrap_err();
        assert_eq!(err, ProgramError::CustomError { val: "couldn't parse id".to_string() });

        let mut data = &[
            vec!["1", "Tom", "10"],
            vec!["2", "Jerry", "15"],
            vec!["4", "Joe", "20"],
        ];
        let err = append_entry_to_data(entry, data, NON_EXISTING_PATH).unwrap_err();
        assert_eq!(err, ProgramError::DataAccessError);
        let res = append_entry_to_data(entry, data, TEST_DATA_PATH).unwrap();
        assert_eq!(res, ());
        // cleanup
        std::fs::remove_file(TEST_DATA_PATH).unwrap();
    }
}