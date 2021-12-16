use crate::batch::BatchAppendClient;
use crate::options::batch_append::BatchAppendOptions;
use crate::options::persistent_subscription::PersistentSubscriptionOptions;
use crate::options::read_all::ReadAllOptions;
use crate::options::read_stream::ReadStreamOptions;
use crate::options::subscribe_to_stream::SubscribeToStreamOptions;
use crate::{
    commands, DeletePersistentSubscriptionOptions, DeleteStreamOptions,
    GetPersistentSubscriptionInfoOptions, ListPersistentSubscriptionsOptions,
    PersistentSubscription, PersistentSubscriptionInfo, PersistentSubscriptionToAllOptions,
    Position, ReplayParkedMessagesOptions, StreamMetadata, StreamMetadataResult,
    SubscribeToAllOptions, SubscribeToPersistentSubscriptionn, Subscription, ToCount,
    TombstoneStreamOptions, VersionedMetadata, WriteResult, WrongExpectedVersion,
};
use crate::{
    grpc::{ClientSettings, GrpcClient},
    Single,
};
use crate::{
    options::append_to_stream::{AppendToStreamOptions, ToEvents},
    EventData,
};

/// Represents a client to a single node. `Client` maintains a full duplex
/// communication to EventStoreDB.
///
/// Many threads can use an EventStoreDB client at the same time
/// or a single thread can make many asynchronous requests.
#[derive(Clone)]
pub struct Client {
    http_client: reqwest::Client,
    client: GrpcClient,
    settings: ClientSettings,
}

impl Client {
    /// Creates a gRPC client to an EventStoreDB database.
    pub fn new(settings: ClientSettings) -> crate::Result<Self> {
        let client = GrpcClient::create(settings.clone());

        let http_client = reqwest::Client::builder()
            .danger_accept_invalid_certs(!settings.is_tls_certificate_verification_enabled())
            .https_only(settings.is_secure_mode_enabled())
            .build()
            .map_err(|e| crate::Error::InitializationError(e.to_string()))?;

        Ok(Client {
            http_client,
            client,
            settings,
        })
    }

    /// Sends events to a given stream.
    pub async fn append_to_stream<Events>(
        &self,
        stream_name: impl AsRef<str>,
        options: &AppendToStreamOptions,
        events: Events,
    ) -> crate::Result<Result<WriteResult, WrongExpectedVersion>>
    where
        Events: ToEvents + 'static,
    {
        commands::append_to_stream(&self.client, stream_name, options, events.into_events()).await
    }

    // Sets a stream metadata.
    pub async fn set_stream_metadata(
        &self,
        stream_name: impl AsRef<str>,
        options: &AppendToStreamOptions,
        metadata: StreamMetadata,
    ) -> crate::Result<Result<WriteResult, WrongExpectedVersion>> {
        let event = EventData::json("$metadata", metadata)
            .map_err(|e| crate::Error::InternalParsingError(e.to_string()))?;

        self.append_to_stream(format!("$${}", stream_name.as_ref()), options, event)
            .await
    }

    // Creates a batch-append client.
    pub fn batch_append(&self, options: &BatchAppendOptions) -> BatchAppendClient {
        commands::batch_append(&self.client, options)
    }

    /// Reads events from a given stream. The reading can be done forward and
    /// backward.
    pub async fn read_stream<Count>(
        &self,
        stream_name: impl AsRef<str>,
        options: &ReadStreamOptions,
        count: Count,
    ) -> crate::Result<Count::Selection>
    where
        Count: ToCount,
    {
        let stream = commands::read_stream(
            self.client.clone(),
            options,
            stream_name,
            count.to_count() as u64,
        )
        .await?;

        Ok(count.select(stream).await)
    }

    /// Reads events for the system stream `$all`. The reading can be done
    /// forward and backward.
    pub async fn read_all<Count>(
        &self,
        options: &ReadAllOptions,
        count: Count,
    ) -> crate::Result<Count::Selection>
    where
        Count: ToCount,
    {
        let stream =
            commands::read_all(self.client.clone(), options, count.to_count() as u64).await?;

        Ok(count.select(stream).await)
    }

    /// Reads a stream metadata.
    pub async fn get_stream_metadata(
        &self,
        stream_name: impl AsRef<str>,
        options: &ReadStreamOptions,
    ) -> crate::Result<StreamMetadataResult> {
        let result = self
            .read_stream(format!("$${}", stream_name.as_ref()), options, Single)
            .await?;

        match result {
            Ok(event) => {
                let event = event.expect("to be defined");
                let metadata = event
                    .get_original_event()
                    .as_json::<StreamMetadata>()
                    .map_err(|e| crate::Error::InternalParsingError(e.to_string()))?;

                let metadata = VersionedMetadata {
                    stream: event.get_original_stream_id().to_string(),
                    version: event.get_original_event().revision,
                    metadata,
                };

                Ok(StreamMetadataResult::Success(Box::new(metadata)))
            }
            Err(e) => match e {
                crate::Error::ResourceNotFound => Ok(StreamMetadataResult::NotFound(
                    stream_name.as_ref().to_string(),
                )),
                crate::Error::ResourceDeleted => Ok(StreamMetadataResult::Deleted(
                    stream_name.as_ref().to_string(),
                )),
                other => Err(other),
            },
        }
    }

    /// Soft deletes a given stream.
    /// Makes use of Truncate before. When a stream is deleted, its Truncate
    /// before is set to the streams current last event number. When a soft
    /// deleted stream is read, the read will return a StreamNotFound. After
    /// deleting the stream, you are able to write to it again, continuing from
    /// where it left off.
    pub async fn delete_stream(
        &self,
        stream_name: impl AsRef<str>,
        options: &DeleteStreamOptions,
    ) -> crate::Result<Option<Position>> {
        commands::delete_stream(&self.client, stream_name, options).await
    }

    /// Hard deletes a given stream.
    /// A hard delete writes a tombstone event to the stream, permanently
    /// deleting it. The stream cannot be recreated or written to again.
    /// Tombstone events are written with the event type '$streamDeleted'. When
    /// a hard deleted stream is read, the read will return a StreamDeleted.
    pub async fn tombstone_stream(
        &self,
        stream_name: impl AsRef<str>,
        options: &TombstoneStreamOptions,
    ) -> crate::Result<Option<Position>> {
        commands::tombstone_stream(&self.client, stream_name, options).await
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
    pub async fn subscribe_to_stream(
        &self,
        stream_name: impl AsRef<str>,
        options: &SubscribeToStreamOptions,
    ) -> Subscription {
        commands::subscribe_to_stream(self.client.clone(), stream_name, options)
    }

    /// Like [`subscribe_to_stream`] but specific to system `$all` stream.
    ///
    /// [`subscribe_to_stream`]: #method.subscribe_to_stream
    pub async fn subscribe_to_all(&self, options: &SubscribeToAllOptions) -> Subscription {
        commands::subscribe_to_all(self.client.clone(), options)
    }

    /// Creates a persistent subscription group on a stream.
    ///
    /// Persistent subscriptions are special kind of subscription where the
    /// server remembers the state of the subscription. This allows for many
    /// different modes of operations compared to a regular or catchup
    /// subscription where the client holds the subscription state.
    pub async fn create_persistent_subscription(
        &self,
        stream_name: impl AsRef<str>,
        group_name: impl AsRef<str>,
        options: &PersistentSubscriptionOptions,
    ) -> crate::Result<()> {
        commands::create_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
        )
        .await
    }

    /// Creates a persistent subscription group on a the $all stream.
    pub async fn create_persistent_subscription_to_all(
        &self,
        group_name: impl AsRef<str>,
        options: &PersistentSubscriptionToAllOptions,
    ) -> crate::Result<()> {
        commands::create_persistent_subscription(&self.client, "", group_name.as_ref(), options)
            .await
    }

    /// Updates a persistent subscription group on a stream.
    pub async fn update_persistent_subscription(
        &self,
        stream_name: impl AsRef<str>,
        group_name: impl AsRef<str>,
        options: &PersistentSubscriptionOptions,
    ) -> crate::Result<()> {
        commands::update_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
        )
        .await
    }

    /// Updates a persistent subscription group to $all.
    pub async fn update_persistent_subscription_to_all(
        &self,
        group_name: impl AsRef<str>,
        options: &PersistentSubscriptionToAllOptions,
    ) -> crate::Result<()> {
        commands::update_persistent_subscription(&self.client, "", group_name.as_ref(), options)
            .await
    }

    /// Deletes a persistent subscription group on a stream.
    pub async fn delete_persistent_subscription(
        &self,
        stream_name: impl AsRef<str>,
        group_name: impl AsRef<str>,
        options: &DeletePersistentSubscriptionOptions,
    ) -> crate::Result<()> {
        commands::delete_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
            false,
        )
        .await
    }

    /// Deletes a persistent subscription group on the $all stream.
    pub async fn delete_persistent_subscription_to_all(
        &self,
        group_name: impl AsRef<str>,
        options: &DeletePersistentSubscriptionOptions,
    ) -> crate::Result<()> {
        commands::delete_persistent_subscription(
            &self.client,
            "",
            group_name.as_ref(),
            options,
            true,
        )
        .await
    }

    /// Connects to a persistent subscription group on a stream.
    pub async fn subscribe_to_persistent_subscription(
        &self,
        stream_name: impl AsRef<str>,
        group_name: impl AsRef<str>,
        options: &SubscribeToPersistentSubscriptionn,
    ) -> crate::Result<PersistentSubscription> {
        commands::subscribe_to_persistent_subscription(
            &self.client,
            stream_name.as_ref(),
            group_name.as_ref(),
            options,
            false,
        )
        .await
    }

    /// Connects to a persistent subscription group to $all stream.
    pub async fn subscribe_to_persistent_subscription_to_all(
        &self,
        group_name: impl AsRef<str>,
        options: &SubscribeToPersistentSubscriptionn,
    ) -> crate::Result<PersistentSubscription> {
        commands::subscribe_to_persistent_subscription(
            &self.client,
            "",
            group_name.as_ref(),
            options,
            true,
        )
        .await
    }

    /// Replays a persistent subscriptions parked events.
    pub async fn replay_parked_messages(
        &self,
        stream_name: impl AsRef<str>,
        group_name: impl AsRef<str>,
        options: &ReplayParkedMessagesOptions,
    ) -> crate::Result<()> {
        let handle = self.client.current_node().await?;

        let mut builder = self
            .http_client
            .post(format!(
                "{}/subscriptions/{}/{}/replayParked",
                handle.url(),
                stream_name.as_ref(),
                group_name.as_ref(),
            ))
            .header("content-type", "application/json")
            .header("content-length", "0");

        if let Some(stop_at) = options.stop_at {
            builder = builder.query(&[("stop_at", stop_at.as_secs().to_string().as_str())])
        }

        builder = http_configure_auth(
            builder,
            options
                .credentials
                .as_ref()
                .or_else(|| self.settings.default_authenticated_user().as_ref()),
        );

        http_execute_request(builder).await?;

        Ok(())
    }

    /// Lists all persistent subscriptions to date.
    pub async fn list_all_persistent_subscriptions(
        &self,
        options: &ListPersistentSubscriptionsOptions,
    ) -> crate::Result<Vec<PersistentSubscriptionInfo>> {
        let handle = self.client.current_node().await?;

        let mut builder = self
            .http_client
            .get(format!("{}/subscriptions", handle.url()))
            .header("content-type", "application/json");

        builder = http_configure_auth(
            builder,
            options
                .credentials
                .as_ref()
                .or_else(|| self.settings.default_authenticated_user().as_ref()),
        );

        let resp = http_execute_request(builder).await?;

        resp.json::<Vec<PersistentSubscriptionInfo>>()
            .await
            .map_err(|e| {
                error!("Error when listing persistent subscriptions: {}", e);
                crate::Error::InternalParsingError(e.to_string())
            })
    }

    /// List all persistent subscriptions of a specific stream.
    pub async fn list_persistent_subscriptions_for_stream(
        &self,
        stream_name: impl AsRef<str>,
        options: &ListPersistentSubscriptionsOptions,
    ) -> crate::Result<Vec<PersistentSubscriptionInfo>> {
        let handle = self.client.current_node().await?;

        let mut builder = self
            .http_client
            .get(format!(
                "{}/subscriptions/{}",
                handle.url(),
                stream_name.as_ref()
            ))
            .header("content-type", "application/json");

        builder = http_configure_auth(
            builder,
            options
                .credentials
                .as_ref()
                .or_else(|| self.settings.default_authenticated_user().as_ref()),
        );

        let resp = http_execute_request(builder).await?;

        resp.json::<Vec<PersistentSubscriptionInfo>>()
            .await
            .map_err(|e| {
                error!("Error when listing persistent subscriptions: {}", e);
                crate::Error::InternalParsingError(e.to_string())
            })
    }

    // Gets a specific persistent subscription info.
    pub async fn get_persistent_subscription_info(
        &self,
        stream_name: impl AsRef<str>,
        group_name: impl AsRef<str>,
        options: &GetPersistentSubscriptionInfoOptions,
    ) -> crate::Result<PersistentSubscriptionInfo> {
        let handle = self.client.current_node().await?;

        let mut builder = self
            .http_client
            .get(format!(
                "{}/subscriptions/{}/{}/info",
                handle.url(),
                stream_name.as_ref(),
                group_name.as_ref(),
            ))
            .header("content-type", "application/json");

        builder = http_configure_auth(
            builder,
            options
                .credentials
                .as_ref()
                .or_else(|| self.settings.default_authenticated_user().as_ref()),
        );

        let resp = http_execute_request(builder).await?;

        resp.json::<PersistentSubscriptionInfo>()
            .await
            .map_err(|e| {
                error!("Error when listing persistent subscriptions: {}", e);
                crate::Error::InternalParsingError(e.to_string())
            })
    }
}

fn http_configure_auth(
    builder: reqwest::RequestBuilder,
    creds_opt: Option<&crate::Credentials>,
) -> reqwest::RequestBuilder {
    if let Some(creds) = creds_opt {
        builder.basic_auth(
            unsafe { std::str::from_utf8_unchecked(creds.login.as_ref()) },
            unsafe { Some(std::str::from_utf8_unchecked(creds.password.as_ref())) },
        )
    } else {
        builder
    }
}

async fn http_execute_request(
    builder: reqwest::RequestBuilder,
) -> crate::Result<reqwest::Response> {
    let resp = builder.send().await.map_err(|e| {
        if let Some(status) = e.status() {
            match status {
                http::StatusCode::UNAUTHORIZED => crate::Error::AccessDenied,
                http::StatusCode::NOT_FOUND => crate::Error::ResourceNotFound,
                code if code.is_server_error() => crate::Error::ServerError(e.to_string()),
                code => {
                    error!(
                        "Unexpected error when dealing with HTTP request to the server: Code={:?}, {}",
                        code,
                        e
                    );
                    crate::Error::InternalClientError
                }
            }
        } else {
            error!(
                "Unexpected error when dealing with HTTP request to the server: {}",
                e,
            );

            crate::Error::InternalClientError
        }
    })?;

    if resp.status().is_success() {
        return Ok(resp);
    }

    let code = resp.status();
    let msg = resp.text().await.unwrap_or_else(|_| "".to_string());

    match code {
        http::StatusCode::UNAUTHORIZED => Err(crate::Error::AccessDenied),
        http::StatusCode::NOT_FOUND => Err(crate::Error::ResourceNotFound),
        code if code.is_server_error() => Err(crate::Error::ServerError(format!(
            "unexpected server error, reason: {:?}",
            code.canonical_reason()
        ))),
        code => {
            error!(
                "Unexpected error when dealing with HTTP request to the server: Code={:?}: {}",
                code, msg,
            );
            Err(crate::Error::InternalClientError)
        }
    }
}
