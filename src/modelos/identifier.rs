use uuid::Uuid;
#[derive(Debug)]
pub struct Identifier {
    // object identifier
    pub(crate) value: String,
}

impl Identifier {
    /// Create a new indentifier
    pub fn new() -> Self {
        let c = Uuid::new_v4();
        Identifier {
            value: c.to_string(),
        }
    }
    /// Create a dummy indentifier
    pub fn default() -> Self {
        Identifier {
            value: String::from(""),
        }
    }

    /// Returns the UUID as a string.
    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}
