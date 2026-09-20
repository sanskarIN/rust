#![crate_name = "issue_95236"]

pub struct ObligationCause;

impl ObligationCause {
    pub fn raise(&self) {}
}
