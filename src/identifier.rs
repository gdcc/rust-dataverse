pub enum Identifier {
    PeristentId(String),
    Id(String),
}

impl Identifier {
    pub fn from_pid_or_id(pid: &Option<String>, id: &Option<String>) -> Self {
        if let Some(pid) = pid {
            Identifier::PeristentId(pid.to_owned())
        } else if let Some(id) = id {
            Identifier::Id(id.to_owned())
        } else {
            panic!("Either a persistent identifier or an identifier must be provided")
        }
    }
}
