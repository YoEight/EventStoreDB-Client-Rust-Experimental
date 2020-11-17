use crate::commands;
use crate::grpc::{ClientSettings, GrpcClient};

/// Represents a connection to a single node. `EventStoreDBConnection` maintains a full duplex
/// connection to the EventStore server. An EventStore connection operates
/// quite differently than say a SQL connection. Normally when you use an
/// EventStore connection you want to keep the connection open for a much
/// longer of time than when you use a SQL connection.
///
/// Another difference is that with the EventStore connection, all operations
/// are handled in a full async manner (even if you call the synchronous
/// behaviors). Many threads can use an EventStore connection at the same time
/// or a single thread can make many asynchronous requests. To get the most
/// performance out of the connection, it is generally recommended to use it
/// in this way.
#[derive(Clone)]
pub struct Client {
    client: GrpcClient,
    settings: ClientSettings,
}

impl Client {
    /// Creates a gRPC connection to an EventStoreDB database.
    pub async fn create(settings: ClientSettings) -> Result<Self, Box<dyn std::error::Error>> {
        let client = GrpcClient::create(settings.clone()).await?;

        Ok(Client { client, settings })
    }
    /// Sends events to a given stream.
    pub fn write_events<S>(&self, stream: S) -> commands::WriteEvents
    where
        S: AsRef<str>,
    {
        commands::WriteEvents::new(
            self.client.clone(),
            stream.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Reads events from a given stream. The reading can be done forward and
    /// backward.
    pub fn read_stream<S>(&self, stream: S) -> commands::ReadStreamEvents
    where
        S: AsRef<str>,
    {
        commands::ReadStreamEvents::new(
            self.client.clone(),
            stream.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Reads events for the system stream `$all`. The reading can be done
    /// forward and backward.
    pub fn read_all(&self) -> commands::ReadAllEvents {
        commands::ReadAllEvents::new(self.client.clone(), self.settings.default_user_name.clone())
    }

    /// Deletes a given stream. By default, the server performs a soft delete,
    /// More information can be found on the [Deleting streams and events]
    /// page.
    ///
    /// [Deleting stream and events]: https://eventstore.org/docs/server/deleting-streams-and-events/index.html
    pub fn delete_stream<S>(&self, stream: S) -> commands::DeleteStream
    where
        S: AsRef<str>,
    {
        commands::DeleteStream::new(
            self.client.clone(),
            stream.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Subscribes to a given stream. This kind of subscription specifies a
    /// starting point (by default, the beginning of a stream). For a regular
    /// stream, that starting point will be an event number. For the system
    /// stream `$all`, it will be a position in the transaction file
    /// (see [`subscribe_to_all_from`]). This subscription will fetch every event
    /// until the end of the stream, then will dispatch subsequently written
    /// events.
    ///
    /// For example, if a starting point of 50 is specified when a stream has
    /// 100 events in it, the subscriber can expect to see events 51 through
    /// 100, and then any events subsequenttly written events until such time
    /// as the subscription is dropped or closed.
    ///
    /// [`subscribe_to_all_from`]: #method.subscribe_to_all_from
    pub fn subscribe_to_stream_from<S>(&self, stream: S) -> commands::RegularCatchupSubscribe
    where
        S: AsRef<str>,
    {
        commands::RegularCatchupSubscribe::new(
            self.client.clone(),
            stream.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Like [`subscribe_to_stream_from`] but specific to system `$all` stream.
    ///
    /// [`subscribe_to_stream_from`]: #method.subscribe_to_stream_from
    pub fn subscribe_to_all_from(&self) -> commands::AllCatchupSubscribe {
        commands::AllCatchupSubscribe::new(
            self.client.clone(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Creates a persistent subscription group on a stream.
    ///
    /// Persistent subscriptions are special kind of subscription where the
    /// server remembers the state of the subscription. This allows for many
    /// different modes of operations compared to a regular or catchup
    /// subscription where the client holds the subscription state.
    pub fn create_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::CreatePersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::CreatePersistentSubscription::new(
            self.client.clone(),
            stream_id.as_ref().to_string(),
            group_name.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Updates a persistent subscription group on a stream.
    pub fn update_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::UpdatePersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::UpdatePersistentSubscription::new(
            self.client.clone(),
            stream_id.as_ref().to_string(),
            group_name.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Deletes a persistent subscription group on a stream.
    pub fn delete_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::DeletePersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::DeletePersistentSubscription::new(
            self.client.clone(),
            stream_id.as_ref().to_string(),
            group_name.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }

    /// Connects to a persistent subscription group on a stream.
    pub fn connect_persistent_subscription<S>(
        &self,
        stream_id: S,
        group_name: S,
    ) -> commands::ConnectToPersistentSubscription
    where
        S: AsRef<str>,
    {
        commands::ConnectToPersistentSubscription::new(
            self.client.clone(),
            stream_id.as_ref().to_string(),
            group_name.as_ref().to_string(),
            self.settings.default_user_name.clone(),
        )
    }
}
