use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringLiterals {
    pub values: Vec<String>,
}

impl StringLiterals {
    pub fn new(values: Vec<String>) -> StringLiterals {
        StringLiterals { values }
    }

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

    pub fn contains(&self, value: &String) -> bool {
        self.values.contains(value)
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.values.get(index)
    }
}
