#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use eventstore::{
    ConnectionSettings, EventData, EventStoreDBConnection, PersistentSubscriptionSettings,
};
use futures::channel::oneshot;
use futures::stream::{self, TryStreamExt};
use std::collections::HashMap;
use std::error::Error;
use tokio_test::block_on;

fn fresh_stream_id(prefix: &str) -> String {
    let uuid = uuid::Uuid::new_v4();

    format!("{}-{}", prefix, uuid)
}

fn generate_events(event_type: String, cnt: usize) -> Vec<EventData> {
    let mut events = Vec::with_capacity(cnt);

    for idx in 1..cnt + 1 {
        let payload = json!({
            "event_index": idx,
        });

        let data = EventData::json(event_type.clone(), payload).unwrap();
        events.push(data);
    }

    events
}

async fn test_write_events(connection: &EventStoreDBConnection) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("write_events");
    let events = generate_events("es6-write-events-test".to_string(), 3);

    let result = connection.write_events(stream_id).send_iter(events).await?;

    debug!("Write response: {:?}", result);

    Ok(())
}

// We read all stream events by batch.
async fn test_read_all_stream_events(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    // Eventstore should always have "some" events in $all, since eventstore itself uses streams, ouroboros style.
    let mut stream = connection
        .read_all()
        .start_from_beginning()
        .execute(1)
        .await?;

    while let Some(_event) = stream.try_next().await? {}

    Ok(())
}

// We read stream events by batch. We also test if we can properly read a
// stream thoroughly.
async fn test_read_stream_events(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("read_stream_events");
    let events = generate_events("es6-read-stream-events-test".to_string(), 10);

    let _ = connection
        .write_events(stream_id.clone())
        .send(stream::iter(events))
        .await?;

    let mut pos = 0usize;
    let mut idx = 0i64;

    let mut stream = connection
        .read_stream(stream_id)
        .start_from_beginning()
        .execute(10)
        .await?;

    while let Some(event) = stream.try_next().await? {
        let event = event.get_original_event();
        let obj: HashMap<String, i64> = event.as_json().unwrap();
        let value = obj.get("event_index").unwrap();

        idx = *value;
        pos += 1;
    }

    assert_eq!(pos, 10);
    assert_eq!(idx, 10);

    Ok(())
}

// We check to see the client can handle the correct GRPC proto response when
// a stream does not exist
async fn test_read_stream_events_non_existent(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("read_stream_events");

    let mut stream = connection
        .read_stream(stream_id)
        .start_from_beginning()
        .execute(1)
        .await?;

    assert!(stream.try_next().await?.is_none());

    Ok(())
}

// We write an event into a stream then delete that stream.
async fn test_delete_stream(connection: &EventStoreDBConnection) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("delete");
    let events = generate_events("delete-test".to_string(), 1);

    let _ = connection
        .write_events(stream_id.clone())
        .send(stream::iter(events))
        .await?;

    let result = connection
        .delete_stream(stream_id.clone())
        .execute()
        .await?;

    debug!("Delete stream [{}] result: {:?}", stream_id, result);

    Ok(())
}

// We write events into a stream. Then, we issue a catchup subscription. After,
// we write another batch of events into the same stream. The goal is to make
// sure we receive events written prior and after our subscription request.
// To assess we received all the events we expected, we test our subscription
// internal state value.
async fn test_subscription(connection: &EventStoreDBConnection) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("catchup");
    let events_before = generate_events("catchup-test-before".to_string(), 3);
    let events_after = generate_events("catchup-test-after".to_string(), 3);

    let _ = connection
        .write_events(stream_id.clone())
        .send(stream::iter(events_before))
        .await?;

    let mut sub = connection
        .subscribe_to_stream_from(stream_id.clone())
        .execute()
        .await?;

    let (tx, recv) = oneshot::channel();

    tokio::spawn(async move {
        let mut count = 0usize;
        let max = 6usize;

        while let Some(_) = sub.try_next().await? {
            count += 1;

            if count == max {
                break;
            }
        }

        tx.send(count).unwrap();
        Ok(()) as eventstore::Result<()>
    });

    let _ = connection
        .write_events(stream_id)
        .send(stream::iter(events_after))
        .await?;

    let test_count = recv.await?;

    assert_eq!(
        test_count, 6,
        "We are testing proper state after catchup subscription: got {} expected {}.",
        test_count, 6
    );

    Ok(())
}

async fn test_create_persistent_subscription(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("create_persistent_sub");

    connection
        .create_persistent_subscription(stream_id, "a_group_name".to_string())
        .execute()
        .await?;

    Ok(())
}

// We test we can successfully update a persistent subscription.
async fn test_update_persistent_subscription(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("update_persistent_sub");

    connection
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .await?;

    let mut setts = PersistentSubscriptionSettings::default();

    setts.max_retry_count = 1000;

    connection
        .update_persistent_subscription(stream_id, "a_group_name".to_string())
        .settings(setts)
        .execute()
        .await?;

    Ok(())
}

// We test we can successfully delete a persistent subscription.
async fn test_delete_persistent_subscription(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("delete_persistent_sub");
    connection
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .await?;

    connection
        .delete_persistent_subscription(stream_id, "a_group_name".to_string())
        .execute()
        .await?;

    Ok(())
}

async fn test_persistent_subscription(
    connection: &EventStoreDBConnection,
) -> Result<(), Box<dyn Error>> {
    let stream_id = fresh_stream_id("persistent_subscription");
    let events = generate_events("es6-persistent-subscription-test".to_string(), 5);

    connection
        .create_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .await?;

    let _ = connection
        .write_events(stream_id.clone())
        .send(stream::iter(events))
        .await?;

    let (mut read, mut write) = connection
        .connect_persistent_subscription(stream_id.clone(), "a_group_name".to_string())
        .execute()
        .await?;

    let max = 10usize;

    let handle = tokio::spawn(async move {
        let mut count = 0usize;
        while let Some(event) = read.try_next().await.unwrap() {
            write.ack_event(event).await.unwrap();

            count += 1;

            if count == max {
                break;
            }
        }

        count
    });

    let events = generate_events("es6-persistent-subscription-test".to_string(), 5);
    let _ = connection
        .write_events(stream_id.clone())
        .send(stream::iter(events))
        .await?;

    let count = handle.await?;

    assert_eq!(
        count, 10,
        "We are testing proper state after persistent subscription: got {} expected {}",
        count, 10
    );

    Ok(())
}

#[test]
fn es6_20_6_test() {
    block_on(async {
        let _ = pretty_env_logger::try_init();
        let settings = "esdb://admin:changeit@localhost:2111,localhost:2112,localhost:2113?tlsVerifyCert=false&nodePreference=leader"
            .parse::<ConnectionSettings>()?;

        let connection = EventStoreDBConnection::create(settings).await?;

        debug!("Before test_write_events…");
        test_write_events(&connection).await?;
        debug!("Complete");
        debug!("Before test_all_read_stream_events…");
        test_read_all_stream_events(&connection).await?;
        debug!("Complete");
        debug!("Before test_read_stream_events…");
        test_read_stream_events(&connection).await?;
        debug!("Complete");
        debug!("Before test_read_stream_events_non_existent");
        test_read_stream_events_non_existent(&connection).await?;
        debug!("Complete");
        debug!("Before test_delete_stream…");
        test_delete_stream(&connection).await?;
        debug!("Complete");
        debug!("Before test_subscription…");
        test_subscription(&connection).await?;
        debug!("Complete");
        debug!("Before test_create_persistent_subscription…");
        test_create_persistent_subscription(&connection).await?;
        debug!("Complete");
        debug!("Before test_update_persistent_subscription…");
        test_update_persistent_subscription(&connection).await?;
        debug!("Complete");
        debug!("Before test_delete_persistent_subscription…");
        test_delete_persistent_subscription(&connection).await?;
        debug!("Complete");
        debug!("Before test_persistent_subscription…");
        test_persistent_subscription(&connection).await?;
        debug!("Complete");

        Ok(()) as Result<(), Box<dyn std::error::Error>>
    })
    .unwrap();
}
