# EventStoreDB Rust Client
![Crates.io](https://img.shields.io/crates/v/eventstore.svg)
![Crates.io](https://img.shields.io/crates/d/eventstore.svg)
![Github action CI workflow](https://github.com/EventStore/EventStoreDB-Client-Rust/workflows/CI/badge.svg?branch=master)
![Discord](https://img.shields.io/discord/415421715385155584.svg)
![Crates.io](https://img.shields.io/crates/l/eventstore.svg)

Official Rust [EventStoreDB] gRPC Client.

[EventStoreDB] is an open-source database built from the ground up for Event Sourcing, with Complex Event Processing in Javascript.

_Note: This client is currently under active development and further API changes are expected. Feedback is very welcome._

## EventStoreDB Server Compatibility
This client is compatible with version `20.6.1` upwards and works on Linux, MacOS and Windows.


Server setup instructions can be found here [EventStoreDB Docs], follow the docker setup for the simplest configuration.

# Example

```rust
use eventstore::EventStoreDBConnection;
use futures::stream::TryStreamExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    is_rust_a_nice_language: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Creates a connection settings for a single node connection.
    let settings = "esdb://admin:changeit@localhost:2113".parse()?;
    let connection = EventStoreDBConnection::create(settings).await?;

    let payload = Foo {
        is_rust_a_nice_language: true,
    };

    // It is not mandatory to use JSON as a data format however EventStoreDB
    // provides great additional value if you do so.
    let evt = EventData::json("language-poll", &payload)?;

    let _ = connection
        .write_events("language-stream")
        .send_event(evt)
        .await?;

    let mut stream = connection
        .read_stream("language-stream")
        .start_from_beginning()
        .read_through()
        .await?;

    while let Some(event) = stream.try_next().await? {
        let event = event.get_original_event()
                .as_json::<Foo>()?;

        // Do something productive with the result.
        println!("{:?}", event);
    }

    Ok(())
}
```

## Support

Information on support can be found here: [EventStoreDB Support]

## Documentation

Documentation for EventStoreDB can be found here: [EventStoreDB Docs]

Bear in mind that this client is not yet properly documented. We are working hard on a new version of the documentation.

## Community

We have a community discussion space at [EventStoreDB Discuss].

[EventStoreDB]: https://eventstore.com/
[eventstoredb docs]: https://developers.eventstore.com/server/20.6/server/installation/
[eventstoredb discuss]: https://discuss.eventstore.com/
[eventstoredb support]: https://eventstore.com/support/