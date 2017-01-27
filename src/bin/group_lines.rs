extern crate clap;
use clap::{Arg, App, ArgMatches};

use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::path::Path;

extern crate anki_convert;
use anki_convert::group_lines::group_lines_file;

fn get_arguments<'a>() -> ArgMatches<'a> {
    App::new("group_lines")
        .version("0.1")
        .author("Grey Hill <infogreytech@gmail.com")
        .about("Converts adjacent lines into single lines.")
        .arg(Arg::with_name("filename")
            .help("The path to the file to convert")
            .required(true)
            .index(1))
        .get_matches()
}

fn main() {
    let arguments = get_arguments();

    let filename = arguments.value_of("filename").unwrap();
    let file_path = &Path::new(filename);
    let mut file = File::open(file_path).expect("Failed to open file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file.");

    let output_filename = filename.to_string() + ".out";
    let output_file_path = &Path::new(&output_filename);
    let mut output_file = File::create(output_file_path).expect("Failed to create file.");

    let new_contents = group_lines_file(&contents);

    output_file.write_all(&new_contents.into_bytes().as_slice())
        .expect("Failed to write to file.");
}
