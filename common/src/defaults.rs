pub const DEFAULT_MAX_STREAMS: u64 = 96 * 1024;
pub const MAX_ALLOWED_PARTIAL_RESPONSES: u64 = DEFAULT_MAX_STREAMS - 1;
pub const DEFAULT_MAX_RECIEVE_WINDOW_SIZE: u64 = 256 * 1024 * 1024; // 256 MBs
pub const DEFAULT_CONNECTION_TIMEOUT: u64 = 10;
pub const DEFAULT_MAX_NB_CONNECTIONS: u64 = 10;
pub const DEFAULT_MAX_ACK_DELAY: u64 = 100;
pub const DEFAULT_ACK_EXPONENT: u64 = 3;
pub const ALPN_GEYSER_PROTOCOL_ID: &[u8] = b"geyser";
pub const MAX_DATAGRAM_SIZE: usize = 1350; // MAX: 65527
