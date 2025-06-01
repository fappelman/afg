use serde::{Deserialize, Serialize};
use crate::string_literals::StringLiterals;
use crate::traits::{Declaration, Instantiate};

impl RadioButton {
    pub fn new(name: String, default: StringLiterals, description: String, choices: StringLiterals) -> RadioButton {
        RadioButton {
            name,
            default,
            description,
            choices
        }
    }

    pub fn map_default(default: &StringLiterals, choices: &StringLiterals) -> StringLiterals {
        // If the default value is a numeric, it should map to the
        // given index in choices
        let mut values = Vec::new();
        for default_value in default.values.iter() {
            let index = default_value.parse::<usize>();
            if index.is_ok() {
                let index = index.unwrap();
                values.push(choices.get(index).unwrap().clone());
            } else {
                values.push(default_value.clone());
            }
        }
        StringLiterals::new(values)
    }

}

impl crate::traits::Result for RadioButton {
    fn result(&self) -> String {
        let statement = format!("{}=\\(flatten({}))",
                                self.name, self.name);
        statement
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadioButton {
    pub name: String,
    pub default: StringLiterals,
    pub description: String,
    pub choices: StringLiterals,
}

impl Instantiate for RadioButton {
    fn instantiate(&self) -> String {
        format!("RadioSelection(label: \"{}\", options: {}, selection: ${})\n",
                self.description,
                self.choices.as_array(),
                self.name)
    }
}

impl Declaration for RadioButton {
    fn declaration(&self) -> String {
        format!("\t@State var {}: Set<String> = {}\n",
                self.name,
                self.default.as_array())
    }
}
