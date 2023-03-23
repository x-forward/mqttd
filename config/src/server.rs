// Server Capability

#[derive(Debug)]
pub struct Server {
    pub cap: Capability,
    pub listen: String,
    pub name: String,
}

impl Server {
    pub fn default() -> Server {
        Server {
            cap: Capability::default(),
            listen: "127.0.0.1:1884".to_string(),
            name: "server-1".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Capability {
    pub maximum_message_expiry_interval: i64,
    pub maximum_client_writes_pending: Option<i32>,
    pub maximum_session_expiry_interval: u32,
    pub maximum_packet_size: u32,
    pub receive_maximum: u16,
    pub topic_alias_maximum: u16,
    pub server_keep_alive: u16,
    pub shared_sub_available: u8,
    pub minimum_protocol_version: u8,
    pub compatibilities: Option<Compatibilities>,
    pub maximum_qos: u8,
    pub retain_available: u8,
    pub wildcard_sub_available: u8,
    pub sub_id_available: u8,
}

// Compatibilities provides flags for using compatibility modes.
#[derive(Debug)]
pub struct Compatibilities {
    pub obscure_not_authorized: bool, // return unspecified errors instead of not authorized
    pub passive_client_disconnect: bool, // don't disconnect the client forcefully after sending disconnect packet (paho)
    pub always_return_response_info: bool, // always return response info (useful for testing)
    pub restore_sys_info_on_restart: bool, // restore system info from store as if server never stopped
}

impl Capability {
    pub fn default() -> Capability {
        Capability {
            maximum_message_expiry_interval: i64::MAX,
            maximum_client_writes_pending: None,
            maximum_session_expiry_interval: 60 * 60 * 24,
            maximum_packet_size: 2,
            receive_maximum: 1,
            topic_alias_maximum: u16::MAX,
            server_keep_alive: 10,
            shared_sub_available: 1,
            minimum_protocol_version: 0x5, // only support mqtt protocol verison 5
            compatibilities: None,
            maximum_qos: 2,
            retain_available: 1,
            wildcard_sub_available: 1,
            sub_id_available: 1,
        }
    }
}
