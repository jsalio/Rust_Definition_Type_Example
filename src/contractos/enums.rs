/// define contract for enums
pub trait EnumString {
    /// Retrieve string enum member value
    /// 
    /// the 'to_str' function take a current value into enum and return this as Inmutable value
    ///
    /// /// # Example
    ///
    /// ```rust
    /// use your_module::{Workflow, Identifier, Status};
    ///
    /// let workflow = Workflow {
    ///     id: Identifier::new(), // Assuming Identifier has a `new` method
    ///     name: String::from("Data Processing"),
    ///     status: Status::InProgress,
    /// };
    ///
    /// println!("Workflow Name: {}", workflow.status.to_str());
    /// ```
    fn to_str(&self) -> &str;
}
