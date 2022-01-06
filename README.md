# EventStoreDB Rust Client
![Crates.io](https://img.shields.io/crates/v/eventstore.svg)
![Crates.io](https://img.shields.io/crates/d/eventstore.svg)
![Github action CI workflow](https://github.com/EventStore/EventStoreDB-Client-Rust/workflows/CI/badge.svg?branch=master)
![Discord](https://img.shields.io/discord/415421715385155584.svg)
![Crates.io](https://img.shields.io/crates/l/eventstore.svg)

Official Rust [EventStoreDB rust gRPC] gRPC Client.

[EventStoreDB] is an open-source database built from the ground up for Event Sourcing, with Complex Event Processing in Javascript.

## EventStoreDB Server Compatibility
This client is compatible with version `20.6.1` upwards and works on Linux, MacOS and Windows.


Server setup instructions can be found here [EventStoreDB Docs], follow the docker setup for the simplest configuration.

# Example

```rust
use eventstore::{ All, Client, EventData, ReadResult };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    is_rust_a_nice_language: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113".parse()?;
    let client = Client::new(settings).await?;

    let payload = Foo {
        is_rust_a_nice_language: true,
    };

    // It is not mandatory to use JSON as a data format however EventStoreDB
    // provides great additional value if you do so.
    let evt = EventData::json("language-poll", &payload)?;

    client
        .append_to_stream("language-stream", &Default::default(), evt)
        .await?;

    let result = client
        .read_stream("language-stream", &Default::default(), All)
        .await?;

	while let Some(event) = stream.next().await? {
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
[EventStoreDB rust gRPC]: https://developers.eventstore.com/clients/grpc/getting-started?codeLanguage=Rust
[eventstoredb docs]: https://developers.eventstore.com/latest.html
[eventstoredb discuss]: https://discuss.eventstore.com/
[eventstoredb support]: https://eventstore.com/support/
