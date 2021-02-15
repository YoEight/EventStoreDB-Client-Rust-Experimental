use crate::grpc::{ClientSettings, GrpcClient};
use crate::options::append_to_stream::{AppendToStreamOptions, ToEvents};
use crate::options::persistent_subscription::PersistentSubscriptionOptions;
use crate::options::read_all::ReadAllOptions;
use crate::options::read_stream::ReadStreamOptions;
use crate::options::subscribe_to_stream::SubscribeToStreamOptions;
use crate::{
    commands, ConnectToPersistentSubscription, DeletePersistentSubscriptionOptions,
    DeleteStreamOptions, Position, ReadResult, SubEvent, SubscribeToAllOptions, SubscriptionRead,
    SubscriptionWrite, ToCount, WriteResult, WrongExpectedVersion,
};
use futures::stream::BoxStream;

/// Represents a client to a single node. `Client` maintains a full duplex
/// communication to EventStoreDB.
///
/// Many threads can use an EventStoreDB client at the same time
/// or a single thread can make many asynchronous requests.
#[derive(Clone)]
pub struct Client {
    client: GrpcClient,
    settings: ClientSettings,
}

impl Client {
    /// Creates a gRPC client to an EventStoreDB database.
    pub async fn create(settings: ClientSettings) -> Result<Self, Box<dyn std::error::Error>> {
        let client = GrpcClient::create(settings.clone()).await?;

        Ok(Client { client, settings })
    }
    /// Sends events to a given stream.
    pub async fn append_to_stream<StreamName, Events>(
        &self,
        stream_name: StreamName,
        options: &AppendToStreamOptions,
        events: Events,
    ) -> crate::Result<Result<WriteResult, WrongExpectedVersion>>
    where
        StreamName: AsRef<str>,
        Events: ToEvents + 'static,
    {
        commands::append_to_stream(&self.client, stream_name, options, events.into_events()).await
    }

    /// Reads events from a given stream. The reading can be done forward and
    /// backward.
    pub async fn read_stream<StreamName, Count>(
        &self,
        stream_name: StreamName,
        options: &ReadStreamOptions,
        count: Count,
    ) -> crate::Result<ReadResult<Count::Selection>>
    where
        StreamName: AsRef<str>,
        Count: ToCount<'static>,
    {
        let result =
            commands::read_stream(&self.client, options, stream_name, count.to_count() as u64)
                .await?;

        match result {
            ReadResult::Ok(stream) => {
                let stream = count.select(stream).await?;

                Ok(ReadResult::Ok(stream))
            }

            ReadResult::StreamNotFound(stream_name) => Ok(ReadResult::StreamNotFound(stream_name)),
        }
    }

    /// Reads events for the system stream `$all`. The reading can be done
    /// forward and backward.
    pub async fn read_all<Count>(
        &self,
        options: &ReadAllOptions,
        count: Count,
    ) -> crate::Result<Count::Selection>
    where
        Count: ToCount<'static>,
    {
        let stream = commands::read_all(&self.client, &options, count.to_count() as u64).await?;

        count.select(stream).await
    }

    /// Deletes a given stream. By default, the server performs a soft delete.
    pub async fn delete_stream<StreamName>(
        &self,
        stream_name: StreamName,
        options: &DeleteStreamOptions,
    ) -> crate::Result<Option<Position>>
    where
        StreamName: AsRef<str>,
    {
        commands::delete_stream(&self.client, stream_name, options).await
    }

    /// Subscribes to a given stream. This kind of subscription specifies a
    /// starting point (by default, the beginning of a stream). For a regular
    /// stream, that starting point will be an event number. For the system
    /// stream `$all`, it will be a position in the transaction file
    /// (see [`subscribe_to_all`]). This subscription will fetch every event
    /// until the end of the stream, then will dispatch subsequently written
    /// events.
    ///
    /// For example, if a starting point of 50 is specified when a stream has
    /// 100 events in it, the subscriber can expect to see events 51 through
    /// 100, and then any events subsequently written events until such time
    /// as the subscription is dropped or closed.
    ///
    /// [`subscribe_to_all`]: #method.subscribe_to_all_from
    pub async fn subscribe_to_stream<'a, StreamName>(
        &self,
        stream_name: StreamName,
        options: &SubscribeToStreamOptions,
    ) -> crate::Result<BoxStream<'a, crate::Result<SubEvent>>>
    where
        StreamName: AsRef<str>,
    {
        commands::subscribe_to_stream(&self.client, stream_name, options).await
    }

    /// Like [`subscribe_to_stream`] but specific to system `$all` stream.
    ///
    /// [`subscribe_to_stream`]: #method.subscribe_to_stream
    pub async fn subscribe_to_all<'a>(
        &self,
        options: &SubscribeToAllOptions,
    ) -> crate::Result<BoxStream<'a, crate::Result<SubEvent>>> {
        commands::subscribe_to_all(&self.client, options).await
    }

    /// Creates a persistent subscription group on a stream.
    ///
    /// Persistent subscriptions are special kind of subscription where the
    /// server remembers the state of the subscription. This allows for many
    /// different modes of operations compared to a regular or catchup
    /// subscription where the client holds the subscription state.
    pub async fn create_persistent_subscription<StreamName, GroupName>(
        &self,
        stream_name: StreamName,
        group_name: GroupName,
        options: &PersistentSubscriptionOptions,
    ) -> crate::Result<()>
    where
        StreamName: AsRef<str>,
        GroupName: AsRef<str>,
    {
        commands::create_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
        )
        .await
    }

    /// Updates a persistent subscription group on a stream.
    pub async fn update_persistent_subscription<StreamName, GroupName>(
        &self,
        stream_name: StreamName,
        group_name: GroupName,
        options: &PersistentSubscriptionOptions,
    ) -> crate::Result<()>
    where
        StreamName: AsRef<str>,
        GroupName: AsRef<str>,
    {
        commands::update_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
        )
        .await
    }

    /// Deletes a persistent subscription group on a stream.
    pub async fn delete_persistent_subscription<StreamName, GroupName>(
        &self,
        stream_name: StreamName,
        group_name: GroupName,
        options: &DeletePersistentSubscriptionOptions,
    ) -> crate::Result<()>
    where
        StreamName: AsRef<str>,
        GroupName: AsRef<str>,
    {
        commands::delete_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
        )
        .await
    }

    /// Connects to a persistent subscription group on a stream.
    pub async fn connect_persistent_subscription<StreamName, GroupName>(
        &self,
        stream_name: StreamName,
        group_name: GroupName,
        options: &ConnectToPersistentSubscription,
    ) -> crate::Result<(SubscriptionRead, SubscriptionWrite)>
    where
        StreamName: AsRef<str>,
        GroupName: AsRef<str>,
    {
        commands::connect_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
        )
        .await
    }
}
