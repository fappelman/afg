use serde::{Deserialize, Serialize};
use crate::traits::{Declaration, Instantiate};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputString {
    pub name: String,
    pub default: String,
    pub description: String,
}

impl InputString {
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

