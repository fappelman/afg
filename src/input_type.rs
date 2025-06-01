use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputType {
    InputString,
    InputBoolean,
    Picker,
    RadioButton,
}

impl InputType {
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
