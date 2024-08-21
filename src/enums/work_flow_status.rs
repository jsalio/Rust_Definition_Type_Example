use super::super::contractos::enums::EnumString;
#[derive(Debug)]
pub enum WorkFlowStatus {
    Enable,
    Archived,
    Locked,
}

impl EnumString for WorkFlowStatus {
    fn to_str(&self) -> &str {
        match self {
            WorkFlowStatus::Enable => "Available",
            WorkFlowStatus::Archived => "Occupied",
            WorkFlowStatus::Locked => "Maintenance",
        }
    }
}

