use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// The different input types
pub enum InputType {
    /// An input field for the type String
    InputString,
    /// An input field for the type Boolean
    InputBoolean,
    /// An input field for the type Picker
    Picker,
    /// An input field for the type radio button
    RadioButton,
}

impl InputType {
    /// Create a new InputType based on the value of the String
    pub fn new(value: &str) -> Result<InputType, String> {
        match value.to_lowercase().trim() {
            "string" => Ok(InputType::InputString),
            "bool" => Ok(InputType::InputBoolean),
            "picker" => Ok(InputType::Picker),
            "radio" => Ok(InputType::RadioButton),
            _ => {
                let msg = format!("Unknown input type: {}", value.to_lowercase());
                Err(msg)
            }
        }
    }
}
