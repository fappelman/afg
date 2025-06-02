mod parse;
mod config;
mod input_type;
mod string_array_argument;
mod traits;
mod radio_button;
mod picker;
mod input_boolean;
mod input_string;
mod temp_file;

use clap::Arg;
use clap::ArgAction;
use std::fs;
use exec;

#[tokio::main]
async fn main() {
    
    let app = clap::Command::new("afg")
        .version("1.0.0")
        .about("Alfred Form Generator")
        .arg(
            Arg::new("field_separator")
                .long("separator")
                .num_args(1)
                .default_value("@")
                .help("Specify the field separator")
        )
        .arg (
            Arg::new("field_height")
                .long("field-height")
                .default_value("35")
                .num_args(1)
                .value_names(&["INT"])
                .help("Specify the field height")
        )
        .arg (
            Arg::new("field_width")
                .long("field-width")
                .default_value("300")
                .num_args(1)
                .value_names(&["INT"])
                .help("Specify the field width")
        )
        .arg (
            Arg::new("window_width")
                .long("window-width")
                .default_value("400")
                .num_args(1)
                .value_names(&["INT"])
                .help("Specify the width of the window")
        )
        .arg (
            Arg::new("field")
                .short('f')
                .long("field")
                .num_args(1)
                .action(ArgAction::Append)
                .required(true)
                .help("Describes details of a field. Can be repeated multiple times.")
        )
        // Positional arguments
        .arg(
            Arg::new("title")
                .required(true)
                .help("Specify the dialog title")
        )
        .after_help("This application is used to generate a swift application \
        that can be run in Alfred as a form input");
    let matches = app.get_matches();

    // Get the necessary input from the command line
    let separator = matches.get_one::<String>("field_separator").unwrap();
    let title = matches.get_one::<String>("title").unwrap();
    let fields : Vec<_> = matches.get_many::<String>("field").unwrap().collect();
    let field_height: u32 = matches.get_one::<String>("field_height").unwrap().parse().unwrap();
    let field_width = matches.get_one::<String>("field_width").unwrap();
    let window_width: u32 = matches.get_one::<String>("window_width").unwrap().parse().unwrap();

    // Convert the fields and title into a configuration
    let configuration = parse::analyze_fields(&fields, separator, title);

    // Process the template
    let template = include_str!("../Template/template.txt");
    let declaration = configuration.variable_declaration(window_width, field_height);
    let instantiate = configuration.instantiate();
    let title = configuration.dialog_title();
    let result = configuration.result();

    let result: String = template
        .replacen("{declaration}", &declaration,1)
        .replacen("{title}", &title,1)
        .replacen("{instantiate}", &instantiate,1)
        .replacen("{result}", &result,1)
        .replacen("{field_width}", &field_width, std::usize::MAX);

    let temp_file = crate::temp_file::temp_file();
    fs::write(&temp_file, &result).expect("Unable to write to temp file");
    let rc = exec::Command::new("swift")
        .arg(temp_file)
        .exec();
    println!("Error: {}", rc);
}
