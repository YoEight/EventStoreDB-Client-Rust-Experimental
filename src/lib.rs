/// Provides a TCP client for [GetEventStore] datatbase.

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod commands;
pub mod connection;
pub mod event_store;
mod gossip;
mod grpc_connection;
pub mod types;

pub use connection::EventStoreDBConnection;
pub use grpc_connection::ConnectionSettings;
pub use types::*;
