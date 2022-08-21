use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use structopt::StructOpt;
use cli_table::{print_stdout, Cell, CellStruct, Style, Table};

mod tests;
mod helpers;
pub mod config;

#[derive(StructOpt, Debug)]
enum SearchCommand {
    Id {
        id: usize,
    },
    Name {
        name: String,
    },
    Email {
        email: String,
    },
}

#[derive(StructOpt, Debug)]
enum EditCommand {
    Name {
        name: String,
    },
    Email {
        email: String,
    },
    NameEmail {
        name: String,
        email: String,
    },
}

/// Command Menu
#[derive(StructOpt, Debug)]
#[structopt(name = "menu")]
enum Opt {
    View,
    Add {
        name: String,
        email: String,
    },
    Search {
        #[structopt(subcommand)]
        sub: SearchCommand,
    },
    Remove {
        id: usize,
    },
    Edit {
        id: usize,
        #[structopt(subcommand)]
        sub: EditCommand,
    },
}

fn display_table<S: AsRef<str>, T: AsRef<str>>(title: &Vec<S>, body: &[Vec<T>]) {
    let processed_title: Vec<CellStruct> = title
        .iter()
        .map(|entry| entry.as_ref().cell().bold(true))
        .collect();
    let processed_body: Vec<Vec<CellStruct>> = helpers::map_data_to_cells(body);
    let table = processed_body.table().title(processed_title).bold(true);
    if let Err(_e) = print_stdout(table) {
        println!("Encountered error while trying to display table");
    }
}

fn append_entry_to_data<S: AsRef<str>, T: AsRef<str>>(entry: (S, S), data: &[Vec<T>], data_path: &str) {
    let name = entry.0.as_ref();
    let email = entry.1.as_ref();
    let id = match &data.iter().last().unwrap()[0].as_ref().parse::<i32>() {
        Ok(idx) => Ok(idx + 1),
        Err(e) => {
            println!("Error getting the id of data: {}", e);
            Err(())
        }
    };
    if let Err(_) = id {
        return;
    }
    println!("Adding id: {}, name: {}, and email {} to data...", id.unwrap(), name, email);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(data_path)
        .unwrap();
    let line = format!("{},{},{}", id.unwrap(), name, email);
    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn edit<S: AsRef<str>, T: AsRef<str>>(data: &[(usize, Vec<S>)], id: T, mut name: Option<T>, mut email: Option<T>, data_path: &str, temp_path: &str) {
    match data.iter().find(|line| line.1[0].as_ref().eq(id.as_ref())) {
        Some((idx, _)) => {
            let file = File::open(data_path);
            match file {
                Ok(file) => {
                    {
                        if Path::new(temp_path).exists() {
                            fs::remove_file(temp_path).unwrap();
                        }
                        let out_file: File = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .append(true)
                            .open(temp_path)
                            .unwrap();
                        let reader = BufReader::new(&file);
                        let mut writer = BufWriter::new(&out_file);

                        for (index, line) in reader.lines().enumerate() {
                            let line = line.as_ref().unwrap();
                            if index != *idx {
                                if let Err(_) = writeln!(writer, "{}", line) {
                                    println!("Some error occurred while trying to access data.");
                                    return;
                                }
                            } else {
                                let name: String = match name.take() {
                                    Some(v) => v.as_ref().to_string(),
                                    None => {
                                        line.split(",").enumerate()
                                            .filter(|(idx, _)| *idx == 1)
                                            .map(|(_, l)| l)
                                            .find(|_| true)
                                            .unwrap()
                                            .to_string()
                                    }
                                };
                                let email: String = match email.take() {
                                    Some(v) => v.as_ref().to_string(),
                                    None => {
                                        line.split(",").enumerate()
                                            .filter(|(idx, _)| *idx == 2)
                                            .map(|(_, l)| l)
                                            .find(|_| true)
                                            .unwrap()
                                            .to_string()
                                    }
                                };
                                if let Err(_) = writeln!(writer, "{}", format!("{},{},{}", id.as_ref(), name, email)) {
                                    println!("Some error occurred while trying to access data.");
                                    return;
                                }
                            }
                        }
                    }
                    if let Err(_) = fs::rename(temp_path, data_path) {
                        println!("Some error occurred while trying to access data.");
                    }
                }
                Err(_) => println!("Some error occurred while trying to access data."),
            }
        }
        None => println!("Couldn't find entry with id {}", id.as_ref())
    }
}

fn remove<S: AsRef<str>, T: AsRef<str>>(data: &[(usize, Vec<S>)], id: T, data_path: &str, temp_path: &str) {
    match data.iter().find(|line| line.1[0].as_ref().eq(id.as_ref())) {
        Some((idx, _)) => {
            let file = File::open(data_path);
            match file {
                Ok(file) => {
                    {
                        if Path::new(temp_path).exists() {
                            fs::remove_file(temp_path).unwrap();
                        }
                        let out_file: File = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .append(true)
                            .open(temp_path)
                            .unwrap();
                        let reader = BufReader::new(&file);
                        let mut writer = BufWriter::new(&out_file);

                        for (index, line) in reader.lines().enumerate() {
                            let line = line.as_ref().unwrap();
                            if index != *idx {
                                if let Err(_) = writeln!(writer, "{}", line) {
                                    println!("Some error occurred while trying to access data.");
                                    return;
                                }
                            }
                        }
                    }
                    if let Err(_) = fs::rename(temp_path, data_path) {
                        println!("Some error occurred while trying to access data.");
                    }
                }
                Err(_) => println!("Some error occurred while trying to access data."),
            }
        }
        None => println!("Couldn't find entry with id {}", id.as_ref())
    }
}

fn find_one<S: AsRef<str>, T: AsRef<str>>(data: &[Vec<S>], col_idx: usize, query: T) {
    let title = &data[0];
    match data.iter()
        .find(|line| line[col_idx].as_ref().eq(query.as_ref()))
        .map(|line| line.iter().map(|s| s.as_ref().to_string()).collect::<Vec<String>>())
    {
        Some(col) => {
            display_table(&title, &[col])
        }
        None => println!("Couldn't find any entry!"),
    }
}

fn find_many<S: AsRef<str>, T: AsRef<str>>(data: &[Vec<S>], col_idx: usize, query: T, contains: Option<bool>) {
    let title = &data[0];
    let data: Vec<Vec<String>> = data.iter()
        .filter(|line| {
            match contains {
                Some(true) => line[col_idx].as_ref().contains(query.as_ref()),
                _ => line[col_idx].as_ref().eq(query.as_ref()),
            }
        })
        .map(|line| line.iter().map(|s| s.as_ref().to_string()).collect::<Vec<String>>())
        .collect();
    if let 0 = data.len() {
        println!("Couldn't find any entry!");
        return;
    }
    display_table(&title, &data[..]);
}

pub fn run(config: config::Config) {
    let opt = Opt::from_args();
    let data: Option<Vec<(usize, Vec<String>)>> = match helpers::read_lines(config.data_path) {
        Ok(lines) => {
            Some(helpers::map_lines_to_data(lines, config.total_num_cols))
        }
        Err(_) => {
            println!("Some error occurred while trying to access data.");
            None
        }
    };
    match opt {
        Opt::View => if let Some(data) = data {
            let data: Vec<Vec<String>> = data.iter().map(|(_, line)| line.clone()).collect();
            display_table(&data[0], &data[1..]);
        }
        Opt::Add { name, email } => if let Some(data) = data {
            let data: Vec<Vec<String>> = data.iter().map(|(_, line)| line.clone()).collect();
            append_entry_to_data((name, email), &data[1..], config.data_path);
        }
        Opt::Search { sub } => if let Some(data) = data {
            let data: Vec<Vec<String>> = data.iter().map(|(_, line)| line.clone()).collect();
            match sub {
                SearchCommand::Id { id } => find_one(&data[..], config.id_idx, id.to_string()),
                SearchCommand::Name { name } => find_many(&data[..], config.name_idx, name, Some(true)),
                SearchCommand::Email { email } => find_many(&data[..], config.email_idx, email, Some(true)),
            }
        }
        Opt::Remove { id } => if let Some(data) = data {
            remove(&data[..], id.to_string(), config.data_path, config.temp_path);
        }
        Opt::Edit { id, sub } => {
            let mut name_opt: Option<String> = None;
            let mut email_opt: Option<String> = None;
            match sub {
                EditCommand::Name { name } => name_opt = Some(name),
                EditCommand::Email { email } => email_opt = Some(email),
                EditCommand::NameEmail { name, email } => {
                    name_opt = Some(name);
                    email_opt = Some(email);
                }
            }
            if let Some(data) = data {
                edit(&data[..], id.to_string(), name_opt, email_opt, config.data_path, config.temp_path);
            }
        }
    }
}