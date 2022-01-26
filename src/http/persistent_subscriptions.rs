use crate::grpc::Handle;
use crate::{
    ClientSettings, GetPersistentSubscriptionInfoOptions, ListPersistentSubscriptionsOptions,
    PersistentSubscriptionInfo, ReplayParkedMessagesOptions,
    RestartPersistentSubscriptionSubsystem,
};

/// Replays a persistent subscriptions parked events.
pub(crate) async fn replay_parked_messages(
    handle: &Handle,
    http_client: &reqwest::Client,
    settings: &ClientSettings,
    stream_name: impl AsRef<str>,
    group_name: impl AsRef<str>,
    options: &ReplayParkedMessagesOptions,
) -> crate::Result<()> {
    let mut builder = http_client
        .post(format!(
            "{}/subscriptions/{}/{}/replayParked",
            handle.url(),
            urlencoding::encode(stream_name.as_ref()),
            urlencoding::encode(group_name.as_ref()),
        ))
        .header("content-type", "application/json")
        .header("content-length", "0");

    if let Some(stop_at) = options.stop_at {
        builder = builder.query(&[("stop_at", stop_at.to_string().as_str())])
    }

    builder = super::http_configure_auth(
        builder,
        options
            .common_operation_options
            .credentials
            .as_ref()
            .or_else(|| settings.default_authenticated_user().as_ref()),
    );

    super::http_execute_request(builder).await?;

    Ok(())
}

/// Lists all persistent subscriptions to date.
pub(crate) async fn list_all_persistent_subscriptions(
    handle: &Handle,
    client: &reqwest::Client,
    settings: &ClientSettings,
    options: &ListPersistentSubscriptionsOptions,
) -> crate::Result<Vec<PersistentSubscriptionInfo>> {
    let mut builder = client
        .get(format!("{}/subscriptions", handle.url()))
        .header("content-type", "application/json");

    builder = super::http_configure_auth(
        builder,
        options
            .common_operation_options
            .credentials
            .as_ref()
            .or_else(|| settings.default_authenticated_user().as_ref()),
    );

    let resp = super::http_execute_request(builder).await?;

    resp.json::<Vec<PersistentSubscriptionInfo>>()
        .await
        .map_err(|e| {
            error!("Error when listing persistent subscriptions: {}", e);
            crate::Error::InternalParsingError(e.to_string())
        })
}

/// List all persistent subscriptions of a specific stream.
pub(crate) async fn list_persistent_subscriptions_for_stream(
    handle: &Handle,
    http_client: &reqwest::Client,
    settings: &ClientSettings,
    stream_name: impl AsRef<str>,
    options: &ListPersistentSubscriptionsOptions,
) -> crate::Result<Vec<PersistentSubscriptionInfo>> {
    let mut builder = http_client
        .get(format!(
            "{}/subscriptions/{}",
            handle.url(),
            urlencoding::encode(stream_name.as_ref())
        ))
        .header("content-type", "application/json");

    builder = super::http_configure_auth(
        builder,
        options
            .common_operation_options
            .credentials
            .as_ref()
            .or_else(|| settings.default_authenticated_user().as_ref()),
    );

    let resp = super::http_execute_request(builder).await?;

    resp.json::<Vec<PersistentSubscriptionInfo>>()
        .await
        .map_err(|e| {
            error!("Error when listing persistent subscriptions: {}", e);
            crate::Error::InternalParsingError(e.to_string())
        })
}

// Gets a specific persistent subscription info.
pub(crate) async fn get_persistent_subscription_info(
    handle: &Handle,
    http_client: &reqwest::Client,
    settings: &ClientSettings,
    stream_name: impl AsRef<str>,
    group_name: impl AsRef<str>,
    options: &GetPersistentSubscriptionInfoOptions,
) -> crate::Result<PersistentSubscriptionInfo> {
    let mut builder = http_client
        .get(format!(
            "{}/subscriptions/{}/{}/info",
            handle.url(),
            urlencoding::encode(stream_name.as_ref()),
            urlencoding::encode(group_name.as_ref()),
        ))
        .header("content-type", "application/json");

    builder = super::http_configure_auth(
        builder,
        options
            .common_operation_options
            .credentials
            .as_ref()
            .or_else(|| settings.default_authenticated_user().as_ref()),
    );

    let resp = super::http_execute_request(builder).await?;

    resp.json::<PersistentSubscriptionInfo>()
        .await
        .map_err(|e| {
            error!("Error when listing persistent subscriptions: {}", e);
            crate::Error::InternalParsingError(e.to_string())
        })
}

pub(crate) async fn restart_persistent_subscription_subsystem(
    handle: &Handle,
    http_client: &reqwest::Client,
    settings: &ClientSettings,
    options: &RestartPersistentSubscriptionSubsystem,
) -> crate::Result<()> {
    let mut builder = http_client
        .post(format!("{}/subscriptions/restart", handle.url(),))
        .header("content-type", "application/json")
        .header("content-length", "0");

    builder = super::http_configure_auth(
        builder,
        options
            .common_operation_options
            .credentials
            .as_ref()
            .or_else(|| settings.default_authenticated_user().as_ref()),
    );

    super::http_execute_request(builder).await?;

    Ok(())
}
