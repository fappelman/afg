use serde::{Deserialize, Serialize};
use crate::string_array_argument::StringArrayArgument;
use crate::traits::{Declaration, Instantiate};

/// Implements a field representing a picker
/// which allows the selection of value from a drop-down menu  
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Picker {
    /// The name of variable. This name is used as export label
    pub name: String,
    /// The default value. The value should be present in the given choices or
    /// an index in the choices array.
    pub default: String,
    /// The description that is used in the UI to present the picker
    pub description: String,
    /// The allowed choices of the picker
    pub choices: StringArrayArgument,
}

impl Picker {
    /// Create a new picker
    pub fn new(name: String, default: String, description: String, choices: StringArrayArgument) -> Picker {
        Picker {
            name,
            default,
            description,
            choices
        }
    }

    /// Map the default value to a `String`.
    ///
    /// The default value can be represented as a literal `String` that is present in
    /// the `choices` array but also as an integer that points to the position in 
    /// the `choices` array.
    ///
    /// # Example
    /// Given that the choices have the following values "One", "Two", "Three"
    /// 
    /// If the default value is "One", the returned value will be "One"
    /// 
    /// If the default value is "1", the returned value will be "Two"
    ///
    pub fn map_default(default: String, choices: &StringArrayArgument) -> String {
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

