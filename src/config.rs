use serde::{Deserialize, Serialize};
use std::str;
use crate::traits::{Declaration, Instantiate, Result};

/// Possible variable types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Variable {
    /// A type to input a `String`
    InputString(crate::input_string::InputString),
    /// A type to input a `Boolean`
    InputBoolean(crate::input_boolean::InputBoolean),
    /// A type to input a value using a `Picker`
    Picker(crate::picker::Picker),
    /// A type to input one or more values using a radio button
    RadioButton(crate::radio_button::RadioButton),
}

/// The configuration as specified on the command line
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// The title of the dialog
    pub title: String,
    /// The variables that make up the dialog
    pub variables: Vec<Variable>,
}

impl Config {
    /// Output the variable declarations at the top of the main view
    /// Return a string that can be injected into the template
    /// at the position '{declaration}'
    pub fn variable_declaration(&self, window_width: u32, row_height: u32) -> String {
        let mut result = String::new();
        for variable in &self.variables {
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

    /// Get the result `String` that is used to output 
    /// the result of the dialog.
    /// The output is in the form of a `Swift` 
    /// print statement.
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
        result.push_str("\", terminator: \"\")");
        result
    }

    /// Return the instantiation of all the field 
    /// that have been defined in the configuration
    pub fn instantiate(&self) -> String {
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
