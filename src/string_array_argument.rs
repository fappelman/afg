use serde::{Deserialize, Serialize};

/// Field that stores a field value that has one or more `String`'s
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringArrayArgument {
    /// The stored `String` values
    pub values: Vec<String>,
}

impl StringArrayArgument {
    /// Create a new `StringArrayArgument` from the given values
    pub fn new(values: Vec<String>) -> StringArrayArgument {
        StringArrayArgument { values }
    }

    /// Export the values as a `Swift` array 
    /// `String`. .
    /// An example output 
    /// ```swift
    /// "[\"word 1\",\"word 2\"]"
    /// ```
    pub fn as_array(&self) -> String {
        let mut result = String::new();
        result.push_str("[");
        let mapped = &self.values
            .iter()
            .map(|x| format!("\"{x}\""))
            .collect::<Vec<String>>()
            .join(",")
            ;
        result.push_str(&mapped);
        result.push_str("]");
        result
    }

    /// Return `true` if the given value is one of
    /// the store values
    pub fn contains(&self, value: &String) -> bool {
        self.values.contains(value)
    }

    /// Return the value with the given index.
    pub fn get(&self, index: usize) -> Option<&String> {
        self.values.get(index)
    }
}
