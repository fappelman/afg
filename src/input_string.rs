use serde::{Deserialize, Serialize};
use crate::traits::{Declaration, Instantiate};

/// Implements a field representing a text field
/// which allows the input of a String
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputString {
    /// The name of variable. This name is used as export label
    pub name: String,
    /// The default value
    pub default: String,
    /// The description that is used in the UI to present the picker
    pub description: String,
}

impl InputString {
    /// Create a new instance of `InputString`
    pub fn new(name: String, default: String, description: String) -> InputString {
        InputString {
            name,
            default,
            description
        }
    }
}

impl crate::traits::Result for InputString {
    fn result(&self) -> String {
        format!("{}=\\({})", self.name, self.name)
    }
}

impl Instantiate for InputString {
    fn instantiate(&self) -> String {
        format!("StringView(text: \"{}\", input: ${})\n",
                self.description, self.name)
    }
}

impl Declaration for InputString {
    fn declaration(&self) -> String {
        format!("\t@State var {}: String = \"{}\"\n", self.name, self.default)
    }
}

