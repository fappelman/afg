use std::collections::HashMap;
use std::process::exit;
use crate::config::{Config, Variable};
pub fn analyze_fields(fields: &Vec<&String>, separator: &String, title: &String) -> Config {
    let mut variables: Vec<Variable> = Vec::new();
    let known_keywords = vec!["type", "name", "default", "decoration", "choices"];
    for field in fields {
        // The variable that is being constructed
        // let mut variable = Variable{}
        // Split the field on the comma's
        let specs = field.split(separator);
        let mut key_values: HashMap<String, String> = HashMap::new();
        for spec in specs {
            let v: Vec<String>  = spec
                .trim()
                .split("=")
                .map(|val| val.to_string())
                .collect();
            if ! known_keywords.contains(&v[0].as_str()) {
                println!("{} is not known keyword", v[0]);
                exit(-1);
            }

            key_values.insert(v[0].clone().to_lowercase(), v[1].clone());
        }
        let name = key_values.get("name").expect("Could not find name");
        let the_type = key_values.get("type").expect("Could not find type");
        let the_default = key_values.get("default").expect("Could not find default");
        let decoration = key_values.get("decoration").expect("Could not find decoration");

        let choices_string = key_values.get("choices");
        let choices: Option<Vec<String>>;
        if choices_string.is_some() {
            choices = Some(choices_string.unwrap().split(",").map(|val| val.to_string()).collect());
        } else {
            choices = None;
        }
        let variable = Variable {
            name: name.clone(),
            var_type: the_type.clone().to_lowercase(),
            default: the_default.clone(),
            choices: choices.clone(),
            description: decoration.clone()
        };
        variables.push(variable);
    }

    // All fields are parsed. Now create a configuration
    Config {
        title: title.to_string(),
        variables,
    }
}
