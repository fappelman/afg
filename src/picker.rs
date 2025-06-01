use serde::{Deserialize, Serialize};
use crate::string_literals::StringLiterals;
use crate::traits::{Declaration, Instantiate};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Picker {
    pub name: String,
    pub default: String,
    pub description: String,
    pub choices: StringLiterals,
}

impl Picker {
    pub fn new(name: String, default: String, description: String, choices: StringLiterals) -> Picker {
        Picker {
            name,
            default,
            description,
            choices
        }
    }

    pub fn map_default(default: String, choices: &StringLiterals) -> String {
        // If the default value is a numeric, it should map to the
        // given index in choices
        let index = default.parse::<usize>();
        if index.is_ok() {
            let index = index.unwrap();
            choices.get(index).unwrap().clone()
        } else {
            default
        }
    }
}

impl crate::traits::Result for Picker {
    fn result(&self) -> String {
        let statement = format!("{}=\\({})",
                                self.name, self.name);
        statement
    }
}

impl Instantiate for Picker {
    fn instantiate(&self) -> String {
        format!("PickerView(title: \"{}\", selectedOption: ${}, options: {})\n",
                self.description,
                self.name,
                self.choices.as_array())
    }
}

impl Declaration for Picker {
    fn declaration(&self) -> String {
        format!("\t@State var {}: String = \"{}\"\n",
                self.name,
                self.default)
    }
}

