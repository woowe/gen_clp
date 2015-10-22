use std::fs::File;
use std::io::{Read};

extern crate html5ever;
extern crate clap;
extern crate regex;

use html5ever::{parse, one_input};
use html5ever::rcdom::RcDom;

use clap::{Arg, App, SubCommand};

use regex::Regex;

struct ModelValues {
    main_values: String,
    comparisons: Vec<String>
}

fn open_file(path: &str, mut buffer: &mut String)-> std::io::Result<()> {
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut buffer));
    Ok(())
}

fn main() {
    let matches = App::new("clp_gen")
                    .version("1.0")
                    .author("Jason Cardinal <jason.brogrammer@gmail.com>")
                    .arg(Arg::with_name("INPUT_FILE")
                         .help("Sets the file to use")
                         .takes_value(true)
                         .required(true)
                         .short("i")
                         .long("input"))
                    .arg(Arg::with_name("VALUES")
                         .help("Sets the values to use")
                         .takes_value(true)
                         .required(true)
                         .short("v")
                         .long("vals"))
                    .get_matches();

    let input_file = match matches.value_of("INPUT_FILE") {
        Some(file_path) => file_path,
        None => panic!("No file specified")
    };

    let values_file = match matches.value_of("VALUES") {
        Some(file_path) => file_path,
        None => panic!("No value file specified")
    };

    let mut input_file_buffer = String::new();
    let mut values_file_buffer = String::new();
    open_file(input_file, &mut input_file_buffer).unwrap();
    open_file(values_file, &mut values_file_buffer).unwrap();

    let re = Regex::new(r"<!--[\s]*[\w-]+[\s]*-->").unwrap();

    for (s, e) in re.find_iter(&input_file_buffer[..]){

        println!("mathc: {:?}", &input_file_buffer[s..e]);
    }
}

