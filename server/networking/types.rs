use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
/// Authorization request.
pub struct AuthenticationPacket {
    pub username: String,
}

