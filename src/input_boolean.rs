use serde::{Deserialize, Serialize};
use crate::traits::{Declaration, Instantiate};

/// Implements a field representing a text field
/// which allows the input of a String
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputBoolean {
    /// The name of variable. This name is used as export label
    pub name: String,
    /// The default value
    pub default: bool,
    /// The description that is used in the UI to present the picker
    pub description: String,
}

impl InputBoolean {
    /// Create a new instance of `InputBoolean`.
    pub fn new(name: String, default: bool, description: String) -> InputBoolean {
        InputBoolean {
            name,
            default,
            description
        }
    }
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

