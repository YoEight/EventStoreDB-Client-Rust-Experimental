//! [PREVIEW] Exposes API to communicate with ES 6 server. ES 6 Server is still in development so
//! expect that interface to change. Enable `es6` feature flag to use it.
pub mod commands;
pub mod connection;
pub mod gossip;
pub mod grpc_connection;
pub mod types;
pub use connection::EventStoreDBConnection;
pub use grpc_connection::{ConnectionSettings, ConnectionSettingsParseError};
pub mod event_store;
