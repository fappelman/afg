use std::fs;
use std::process::exit;
use serde::{Deserialize, Serialize};
use std::str;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: String,
    pub default: String,
    pub description: String,
    pub choices: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub title: String,
    pub variables: Vec<Variable>,
}

impl Config {
    pub fn declaration(self) -> String {
        let mut result = String::new();
        for variable in self.variables {
            // Get the default value if provided from the command line. If not
            // get it from JSON
            match variable.var_type.as_str() {
                "string" => {
                    let statement = format!("@State var {}: String = \"{}\"\n", variable.name, variable.default);
                    result.push_str(statement.as_str());
                }
                "bool" => {
                    let statement = format!("@State var {}: Bool = {}\n", variable.name, variable.default);
                    result.push_str(statement.as_str());
                }
                "picker" => {
                    let default = Self::enum_letter_string(variable.default);
                    let statement = format!("@State var {}: {} = .{}\n",
                                            variable.name,
                                            variable.name.to_uppercase(),
                                            default);
                    result.push_str(statement.as_str());
                }
                _ => {
                    println!("Unknown variable type in declaration: {}", variable.var_type);
                    exit(-1);
                },
            }
        }
        result
    }

    pub fn dialog_title(self) -> String {
        format!("Title(text: \"{}\")", self.title)
    }

    pub fn result(self) -> String {
        let mut result = String::new();
        result.push_str("print(\"");
        let mut statements: Vec<String> = Vec::new();
        for variable in self.variables {
            match variable.var_type.as_str() {
                "string" => {
                    let statement = format!("{}=\\({})",
                                            variable.name, variable.name);
                    statements.push(statement);
                }
                "bool" => {
                    let statement = format!("{}=\\({})",
                                            variable.name, variable.name);
                    statements.push(statement);
                }
                "picker" => {
                    let statement = format!("{}=\\({})",
                                            variable.name, variable.name);
                    statements.push(statement);
                }
                _ => {
                    println!("Unknown variable type in result: {}", variable.var_type);
                    exit(-1);
                },
            }
        }
        let joined = statements.join("\\t");
        result.push_str(joined.as_str());
        result.push_str("\")");
        result
    }

    pub fn enum_letter_i32(offset: i32) -> char {
        ('A' as u8 + offset as u8) as char
    }

    pub fn enum_letter_string(offset: String) -> char {
        let offset = offset.parse::<i32>().unwrap();
        ('A' as u8 + offset as u8) as char
    }

    pub fn types(self) -> String {
        let mut result = String::new();
        for variable in self.variables {
            if variable.choices.is_some() {
                let choices = variable.choices.unwrap();
                let declaration = format!("enum {}: String, CaseIterable {}\n", variable.name.to_uppercase(), "{");
                result.push_str(declaration.as_str());
                result.push_str("var id: Self { self }\n");
                let mut counter = 0;
                for choice in choices {
                    let declaration = format!("case {} = \"{}\"\n",Self::enum_letter_i32(counter), choice);
                    counter += 1;
                    result.push_str(declaration.as_str());
                }
                result.push_str("}\n");
            }
        }
        result

    }

    pub fn selectors(self, template_directory: String) -> String {
        let mut result = String::new();
        for variable in self.variables {
            if variable.choices.is_some() {
                let path = format!("{}/selector.txt", template_directory);
                let template: String = fs::read_to_string(path).unwrap();
                let enumeration = variable.name.to_uppercase();
                let output: String = template.replacen("{enumeration}",
                                                       enumeration.to_uppercase().as_str(), 2);
                let choices = &variable.choices.unwrap();
                let mut counter = 0;
                let mut cases = String::new();
                for _ in choices {
                    let letter = ('A' as u8 + counter as u8) as char;
                    counter += 1;
                    let declaration = format!("Text(CHOICE.{}.rawValue).tag({}.{})\n",letter, variable.name.to_uppercase(),letter);
                    cases.push_str(declaration.as_str());
                }
                let output: String = output.replacen("{picker}",
                                                     cases.as_str(),
                                                     1);
                result.push_str(output.as_str());
            }
        }
        result
    }

    pub fn rows(self) -> String {
        let mut result = String::new();
        for variable in self.variables {
            match variable.var_type.as_str() {
                "string" => {
                    let statement = format!("Row(text: \"{}\", input: ${})\n",
                                            variable.description, variable.name);
                    result.push_str(statement.as_str());
                }
                "bool" => {
                    let statement = format!("Toggle(\"{}\", isOn: ${})\n
                      .toggleStyle(SwitchToggleStyle())\n
                      .frame(width: 325, alignment: .leading)\n",
                                            variable.description, variable.name);
                    result.push_str(statement.as_str());
                }
                "picker" => {
                    let statement = format!("Selector{}(info: \"{}\", option: ${})\n",
                                            variable.name.to_uppercase(),
                                            variable.description,
                                            variable.name);
                    result.push_str(statement.as_str());
                }
                _ => {
                    println!("Unknown variable type in rows: {}", variable.var_type);
                    exit(-1);
                },
            }
        }
        result
    }
}
