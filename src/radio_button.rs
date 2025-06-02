use serde::{Deserialize, Serialize};
use crate::string_array_argument::StringArrayArgument;
use crate::traits::{Declaration, Instantiate};

/// Implements a field representing a radio button
/// which allows the selection of one or more values   
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadioButton {
    /// The name of variable. This name is used as export label
    pub name: String,
    /// The default values. Each value should be present in the given choices or
    /// an index in the choices array.
    pub default: StringArrayArgument,
    /// The description that is used in the UI to present the radio button
    pub description: String,
    /// The allowed choices of the radio button
    pub choices: StringArrayArgument,
}

impl RadioButton {
    /// Create a new instance of `RadioButton`
    pub fn new(name: String, default: StringArrayArgument, description: String, choices: StringArrayArgument) -> RadioButton {
        RadioButton {
            name,
            default,
            description,
            choices
        }
    }

    /// Map the default values to `Strings`.
    /// 
    /// The default value can be represented as a literal `String` that is present in
    /// the `choices` array but also as an integer that points to the position in 
    /// the `choices` array.
    /// 
    /// # Example
    /// Given that the choices have the following values "One", "Two", "Three"
    /// Give that the default values are "One", "2"
    /// The returned values will be "One", "Two"
    /// 
    pub fn map_default(default: &StringArrayArgument, choices: &StringArrayArgument) -> StringArrayArgument {
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
        StringArrayArgument::new(values)
    }

}

impl crate::traits::Result for RadioButton {
    fn result(&self) -> String {
        let statement = format!("{}=\\(flatten({}))",
                                self.name, self.name);
        statement
    }
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
