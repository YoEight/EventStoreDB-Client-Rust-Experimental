//! Official Rust [EventStoreDB] gRPC Client.
//!
//! [EventStoreDB] is an open-source database built from the ground up for Event Sourcing, with Complex Event Processing in Javascript.
//!
//! _Note: This client is currently under active development and further API changes are expected. Feedback is very welcome._
//!
//! ## EventStoreDB Server Compatibility
//! This client is compatible with version `20.6.1` upwards and works on Linux, MacOS and Windows.
//!
//!
//! Server setup instructions can be found here [EventStoreDB Docs], follow the docker setup for the simplest configuration.
//!
//! # Example
//!
//! ```no_run
//! use eventstore::{ EventStoreDBConnection, EventData };
//! use futures::stream::TryStreamExt;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! struct Foo {
//!     is_rust_a_nice_language: bool,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!     // Creates a connection settings for a single node connection.
//!     let settings = "esdb://admin:changeit@localhost:2113".parse()?;
//!     let connection = EventStoreDBConnection::create(settings).await?;
//!
//!     let payload = Foo {
//!         is_rust_a_nice_language: true,
//!     };
//!
//!     // It is not mandatory to use JSON as a data format however EventStoreDB
//!     // provides great additional value if you do so.
//!     let evt = EventData::json("language-poll", &payload)?;
//!
//!     let _ = connection
//!         .write_events("language-stream")
//!         .send_event(evt)
//!         .await?;
//!
//!     let mut stream = connection
//!         .read_stream("language-stream")
//!         .start_from_beginning()
//!         .read_through()
//!         .await?;
//!
//!     while let Some(event) = stream.try_next().await? {
//!         let event = event.get_original_event()
//!             .as_json::<Foo>()?;
//!
//!         // Do something productive with the result.
//!         println!("{:?}", event);
//!     }
//!
//!     Ok(())
//! }
//! ```
//! [EventStoreDB]: https://eventstore.com/
//! [eventstoredb docs]: https://developers.eventstore.com/server/20.6/server/installation/

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod commands;
mod connection;
mod event_store;
mod gossip;
mod grpc_connection;
mod types;

pub use connection::EventStoreDBConnection;
pub use grpc_connection::ConnectionSettings;
pub use types::*;
