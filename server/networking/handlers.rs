use std::sync::Arc;

use crate::networking::types::AuthenticationPacket;
use crate::logging;
use socketioxide::{adapter::LocalAdapter, Socket};

pub async fn handler(socket: Arc<Socket<LocalAdapter>>, data: Option<AuthenticationPacket>) {
    logging::info(format!("Socket connected on / with id: {}", socket.id));
    if let Some(data) = data {
        logging::debug(format!("Nickname: {:?}", data.username));
        socket.extensions.insert(data.username);
        socket.emit("system", "CONN_SUCCESS").ok();
        socket.join("default").unwrap();
    } else {
        logging::debug("Unauthorized. Disconnecting");
        socket.disconnect().ok();
        return;
    }

}
