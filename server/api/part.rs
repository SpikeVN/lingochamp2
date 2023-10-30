use crate::api::status::Status;

/// Represents the logic of a particular part of a show.
pub trait Part {
    /// Called once per tick, this function contains the logic of the part.
    fn on_update(&mut self) -> Status;
}
