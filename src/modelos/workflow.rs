use super::identifier::Identifier;
use crate::enums::work_flow_status::WorkFlowStatus;
/// Represents a workflow within the system.
///
/// The `Workflow` struct contains information about a particular workflow,
/// including its identifier, name, and current status.
///
/// # Fields
///
/// * `id` - A unique identifier for the workflow. This is of type `Identifier`.
/// * `name` - The name of the workflow as a `String`.
/// * `status` - The current status of the workflow. This is of type `Status`.
///
/// # Example
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
/// println!("Workflow Name: {}", workflow.name);
/// ```
///
/// # Notes
///
/// - The `Identifier` and `Status` types must be defined elsewhere in your code.
/// - Ensure proper handling of the workflow status throughout its lifecycle.
#[derive(Debug)]
pub struct Workflow {
    pub id: Identifier,
    pub(crate) name: String,
    pub(crate) state: WorkFlowStatus,
}

impl Workflow {
    pub fn new(name: String, state: WorkFlowStatus) -> Result<Self, &'static str> {
        match name.is_empty() {
            true => Err("the name is empty"),
            false => Ok(Self {
                id: Identifier::new(),
                name: name,
                state,
            }),
        }
    }

    pub fn defaul() -> Self {
        Self {
            id: Identifier::default(),
            name: String::from(""),
            state: WorkFlowStatus::Archived,
        }
    }
}

impl PartialEq for Workflow {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Workflow {}
