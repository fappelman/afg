/// Produce a statement that can be used to print the final value
/// of the field 
pub trait Result
{
    /// Produce the result statement for the given field
    /// and return as a `String`
    fn result(&self) -> String;
}

/// Produce a statement that creates a given field
pub trait Instantiate {
    /// Produce an instantiation statement for the given field
    /// and return as a `String`
    fn instantiate(&self) -> String;
}

/// Produce a statement that declares a variable for a given field
pub trait Declaration {
    /// Produce a statement that declares a given variable and
    /// return it as a `String`
    fn declaration(&self) -> String;
}
