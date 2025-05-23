mod fields;
mod config;

use clap::Arg;
use clap::ArgAction;
use clap::Command;
use std::fs;
use crate::fields::analyze_fields;


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
            Arg::new("template_directory")
                .long("templates")
                .num_args(1)
                .default_value("Template")
                .help("Specify the directory of templates")
        )
        .arg(
            Arg::new("title")
                .long("title")
                .short('t')
                .num_args(1)
                .required(true)
                .help("Specify the field separator")
        )
        .arg (
            Arg::new("field")
                .short('f')
                .long("field")
                .num_args(1)
                .action(ArgAction::Append)
                .required(true)
                .help("Describes details of a field")
        )
        .arg(
            Arg::new("output").help("The name of the swift output file")
                .required(true),
        )
        .after_help("This application is used to generate a swift application \
        that can be run in Alfred as a form input");
    let matches = app.get_matches();
    let output = matches.get_one::<String>("output").unwrap();
    let template_directory = matches.get_one::<String>("template_directory").unwrap();
    let separator = matches.get_one::<String>("field_separator").unwrap();
    let title = matches.get_one::<String>("title").unwrap();
    let fields : Vec<_> = matches.get_many::<String>("field").unwrap().collect();
    let configuration = analyze_fields(&fields, separator, title);
    let path = format!("{}/template.txt", template_directory);
    let template: String = fs::read_to_string(path).unwrap();
    let result: String = template.replacen("{declaration}", configuration.clone().declaration().as_str(), 1);
    let result: String = result.replacen("{title}", &configuration.clone().dialog_title(), 1);
    let result: String = result.replacen("{rows}", &configuration.clone().rows().as_str(), 1);
    let result: String = result.replacen("{result}", &configuration.clone().result().as_str(), 1);
    let result: String = result.replacen("{types}", &configuration.clone().types().as_str(), 1);
    let result: String = result.replacen("{selectors}", &configuration.clone().selectors(template_directory.to_string()).as_str(), 1);
    fs::write(output, result).expect("Unable to write file");
}
