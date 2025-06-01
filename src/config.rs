use serde::{Deserialize, Serialize};
use std::str;
use crate::traits::{Declaration, Instantiate, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Variable {
    InputString(crate::input_string::InputString),
    InputBoolean(crate::input_boolean::InputBoolean),
    Picker(crate::picker::Picker),
    RadioButton(crate::radio_button::RadioButton),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub title: String,
    pub variables: Vec<Variable>,
}

impl Config {
    /// Output the variable declarations at the top of the main view
    /// Return a string that can be injected into the template
    /// at the position '{declaration}'
    pub fn variable_declaration(self, window_width: u32, row_height: u32) -> String {
        let mut result = String::new();
        for variable in self.variables {
            match variable {
                Variable::InputString(input_string) => {
                    result.push_str(input_string.declaration().as_str());
                }
                Variable::InputBoolean(input_boolean) => {
                    result.push_str(input_boolean.declaration().as_str());
                }
                Variable::Picker(picker) => {
                    result.push_str(picker.declaration().as_str());
                }
                Variable::RadioButton(radio_button) => {
                    result.push_str(radio_button.declaration().as_str());
                }
            }
        }
        let height = format!("\
        let row_height: CGFloat = {}
        let window_width: CGFloat = {}
        ", row_height, window_width);
        result.push_str(height.as_str());
        result
    }

    /// Output the title declaration that can be injected
    /// into the template at the '{title}' position
    pub fn dialog_title(&self) -> String {
        format!("Title(text: \"{}\")", self.title)
    }

    pub fn result(&self) -> String {
        let mut result = String::new();
        result.push_str("print(\"");
        let mut statements: Vec<String> = Vec::new();
        for variable in &self.variables {
            match variable {
                Variable::InputString(input_string) => {
                    statements.push(input_string.result());
                }
                Variable::InputBoolean(input_boolean) => {
                    statements.push(input_boolean.result());
                }
                Variable::Picker(picker) => {
                    statements.push(picker.result());
                }
                Variable::RadioButton(radio_button) => {
                    statements.push(radio_button.result());
                }
            }
        }
        let joined = statements.join("\\t");
        result.push_str(joined.as_str());
        result.push_str("\")");
        result
    }


    pub fn rows(&self) -> String {
        let mut result = String::new();
        let mut first = true;
        for variable in &self.variables {
            match variable {
                Variable::InputString(input_string) => {
                    result.push_str(input_string.instantiate().as_str());
                }
                Variable::InputBoolean(input_boolean) => {
                    result.push_str(input_boolean.instantiate().as_str());
                }
                Variable::Picker(picker) => {
                    result.push_str(picker.instantiate().as_str());
                }
                Variable::RadioButton(radio_button) => {
                    result.push_str(radio_button.instantiate().as_str());
                }
            }
            result.push_str("\t.frame(height: row_height)\n");
            if first {
                result.push_str("\
                \t.focused($focusedField, equals: .field)
                .task {
                   self.focusedField = .field
                }
                ");
                first = false;
            }
        }
        result
    }
}
