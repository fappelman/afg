use std::collections::HashMap;
use std::process::exit;
use crate::config::{Config, Variable};
use crate::input_type::InputType;
use crate::string_array_argument::StringArrayArgument;

/// Analyze the fields as specified on the command line and convert to the
/// a configuration.
pub fn analyze_fields(fields: &Vec<&String>, separator: &String, title: &String) -> Config {
    let mut variables: Vec<Variable> = Vec::new();
    let known_keywords = vec!["type", "name", "default", "decoration", "choices"];
    for field in fields {
        // The variable that is being constructed

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
        let name = name.to_string();
        let the_type = key_values.get("type").expect("Could not find type");
        let the_default = key_values.get("default").expect("Could not find default");
        let default = the_default.to_string();
        let description = key_values.get("decoration").expect("Could not find decoration");
        let description = description.to_string();
        let input_type = InputType::new(the_type).expect("Unknown type specified");
        let choices_string = key_values.get("choices");

        match input_type {
            InputType::InputString => {
                let input_string = crate::input_string::InputString::new(name, default, description);
                variables.push(Variable::InputString(input_string));
            }
            InputType::InputBoolean => {
                let default = default.parse::<bool>().expect("Invalid default value");
                let input_bool = crate::input_boolean::InputBoolean::new(name, default, description);
                variables.push(Variable::InputBoolean(input_bool));
            }
            InputType::Picker => {
                let values = choices_string
                    .unwrap()
                    .split(",")
                    .map(|val| val.to_string())
                    .collect();
                let choices = StringArrayArgument::new(values);
                // Map the default value if required
                let default = crate::picker::Picker::map_default(default, &choices);
                // Make certain that all default values are present in the choices
                if !choices.contains(&default) {
                    panic!("Invalid default value {default} for {name}");
                }

                let picker = crate::picker::Picker::new(name, default, description, choices);
                variables.push(Variable::Picker(picker));
            }
            InputType::RadioButton => {
                let values = choices_string
                    .unwrap()
                    .split(",")
                    .map(|val| val.to_string())
                    .collect();
                let choices = StringArrayArgument::new(values);
                let default = default
                    .split(",")
                    .map(|val| val.to_string())
                    .collect();
                let default = StringArrayArgument::new(default);
                // Map the default value if required
                let default = crate::radio_button::RadioButton::map_default(&default, &choices);
                // Make certain that all default values are present in the choices
                for default_value in default.values.iter() {
                    if !choices.contains(default_value) {
                        panic!("Invalid default value {default_value} for {name}");
                    }
                }
                let radio = crate::radio_button::RadioButton::new(name, default, description, choices);
                variables.push(Variable::RadioButton(radio));
            }
        }
    }

    // All fields are parsed. Now create a configuration
    Config {
        title: title.to_string(),
        variables,
    }
}
