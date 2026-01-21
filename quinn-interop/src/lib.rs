use std::sync::Arc;

use quinn::TransportConfig;

pub fn transport_config() -> Arc<TransportConfig> {
    let transport_config = TransportConfig::default();
    Arc::new(transport_config)
}

pub const SUPPORTED_TESTS: &[&str] = &[
    "http3",
    "handshake",
    "transfer",
    "longrtt",
    "chacha20",
    "multiplexing",
    "retry",
    "resumption",
    "zerortt",
    "blackhole",
    "ecn",
    "amplificationlimit",
    "transferloss",
    "multiconnect",
    "transfercorruption",
    "ipv6",
    "goodput",
    "keyupdate",
    "crosstraffic",
];
