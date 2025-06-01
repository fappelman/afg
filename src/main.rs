mod parse;
mod config;
mod input_type;
mod string_literals;
mod traits;
mod radio_button;
mod picker;
mod input_boolean;
mod input_string;

use clap::Arg;
use clap::ArgAction;
use clap::Command;
use std::fs;
use std::env;

#[tokio::main]
async fn main() {
    
    let app = Command::new("afg")
        .version("1.0.0")
        .about("Alfred Form Generator")
        .arg(
            Arg::new("verbose")
                .short('v')
                .num_args(0)
                .help("turns on verbose mode")
        )
        .arg(
            Arg::new("field_separator")
                .long("separator")
                .num_args(1)
                .default_value("@")
                .help("Specify the field separator")
        )
        .arg(
            Arg::new("template_file")
                .long("template")
                .num_args(1)
                .default_value("Template/template.txt")
                .help("Specify the template file. Can be overruled by the environment variable AFG_TEMPLATE")
        )
        .arg(
            Arg::new("title")
                .long("title")
                .short('t')
                .num_args(1)
                .required(true)
                .help("Specify the dialog title")
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
        .arg(
            Arg::new("output").help("The name of the swift output file")
                .required(true),
        )
        .after_help("This application is used to generate a swift application \
        that can be run in Alfred as a form input");
    let matches = app.get_matches();
    let output = matches.get_one::<String>("output").unwrap();
    let mut template_file = matches.get_one::<String>("template_file").unwrap().to_string();
    let separator = matches.get_one::<String>("field_separator").unwrap();
    let title = matches.get_one::<String>("title").unwrap();
    let fields : Vec<_> = matches.get_many::<String>("field").unwrap().collect();
    let configuration = parse::analyze_fields(&fields, separator, title);
    if env::var("AFG_TEMPLATE").is_ok() {
        template_file = env::var("AFG_TEMPLATE").unwrap();
    }
    let template = match fs::read_to_string(&template_file) {
        Ok(template) => template,
        Err(code) => {
            panic!("AFG_TEMPLATE error for {}: {}", template_file, code);
        }
    };
    // let template: String = fs::read_to_string(template_file).unwrap();
    let height: u32 = matches.get_one::<String>("field_height").unwrap().parse().unwrap();
    let width: u32 = matches.get_one::<String>("window_width").unwrap().parse().unwrap();
    let result: String = template.replacen("{declaration}",
                                           configuration.clone().variable_declaration(width, height).as_str(), 1);
    let result: String = result.replacen("{title}", &configuration.dialog_title(), 1);
    let result: String = result.replacen("{rows}", &configuration.rows().as_str(), 1);
    let result: String = result.replacen("{result}", &configuration.clone().result().as_str(), 1);
    fs::write(output, result).expect("Unable to write to output file");
}
