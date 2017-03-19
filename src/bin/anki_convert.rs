extern crate clap;
use clap::{Arg, App, ArgMatches};

use std::io::Read;
use std::io::Write;
use std::fs::File;

use std::path::Path;

extern crate anki_convert;

fn get_arguments<'a>() -> ArgMatches<'a> {
    App::new("Incremental Reading Convert")
        .version("0.1")
        .author("Grey Hill <infogreytech@gmail.com")
        .about("Converts html files into files reading to be entered \
                into Anki's HTML editor.")
        .arg(Arg::with_name("filename")
            .help("The name of the file to convert")
            .required(true)
            .index(1))
        .get_matches()
}

fn main() {
    let matches = get_arguments();

    let filename = matches.value_of("filename").unwrap();
    let file_path = &Path::new(filename);
    let mut file = File::open(file_path).expect("Unable to open file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file.");

    let file_type = filename.split('.').last()
        .expect("Failed to get filetype.");

    let new_contents = anki_convert::convert_file(&file_type, &contents);

    let output_filename = filename.to_string() + ".out";
    let output_file_path = &Path::new(&output_filename);
    let mut output_file = File::create(output_file_path)
        .expect("Failed to create file.");

    output_file.write_all(&new_contents.into_bytes().as_slice())
        .expect("Failed to write to file.");
}
