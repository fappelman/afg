use serde::{Deserialize, Serialize};
use crate::traits::{Declaration, Instantiate};

impl InputBoolean {
    pub fn new(name: String, default: bool, description: String) -> InputBoolean {
        InputBoolean {
            name,
            default,
            description
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputBoolean {
    pub name: String,
    pub default: bool,
    pub description: String,
}

impl crate::traits::Result for InputBoolean {
    fn result(&self) -> String {
        let statement = format!("{}=\\({})",
                                self.name, self.name);
        statement
    }
}

impl Instantiate for InputBoolean {
    fn instantiate(&self) -> String {
        format!("ToggleView(isOn: ${}, label: \"{}\")\n",
                self.name, self.description)
    }
}

impl Declaration for InputBoolean {
    fn declaration(&self) -> String {
        format!("\t@State var {}: Bool = {}\n", self.name, self.default)
    }
}

