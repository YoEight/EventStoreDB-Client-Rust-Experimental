#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_results)]
#![allow(unused_variables)]
#![allow(dead_code)]
use eventstore::{
    Client, PersistentSubEvent, PersistentSubscriptionOptions, PersistentSubscriptionToAllOptions,
    SubscriptionFilter,
};

async fn create_persistent_subscription(client: &Client) -> eventstore::Result<()> {
    // #region create-persistent-subscription-to-stream
    client
        .create_persistent_subscription("test-stream", "subscription-group", &Default::default())
        .await?;
    // #endregion create-persistent-subscription-to-stream

    Ok(())
}

async fn connect_to_persistent_subscription_to_stream(client: &Client) -> eventstore::Result<()> {
    // #region subscribe-to-persistent-subscription-to-stream
    let (mut read, mut write) = client
        .subscribe_to_persistent_subscription(
            "test-stream",
            "subscription-group",
            &Default::default(),
        )
        .await?;

    while let Some(PersistentSubEvent {
        retry_count,
        event: inner,
    }) = read.try_next_event().await?
    {
        // Doing some productive work with the event...
        let _ = write.ack_event(inner).await;
    }
    // #endregion subscribe-to-persistent-subscription-to-stream
    Ok(())
}

async fn create_persistent_subscription_to_all(client: &Client) -> eventstore::Result<()> {
    // #region create-persistent-subscription
    let options = PersistentSubscriptionToAllOptions::default()
        .filter(SubscriptionFilter::on_stream_name().add_prefix("test"));

    client
        .create_persistent_subscription_to_all("subscription-group", &options)
        .await?;
    // #endregion create-persistent-subscription
    Ok(())
}

async fn connect_to_persistent_subscription_with_manual_acks(
    client: &Client,
) -> eventstore::Result<()> {
    // #region subscribe-to-persistent-subscription-with-manual-acks
    let (mut read, mut write) = client
        .subscribe_to_persistent_subscription(
            "test-stream",
            "subscription-group",
            &Default::default(),
        )
        .await?;

    while let Some(PersistentSubEvent {
        event: inner,
        retry_count,
    }) = read.try_next_event().await?
    {
        // Doing some productive work with the event...
        let _ = write.ack_event(inner).await;
    }
    // #endregion subscribe-to-persistent-subscription-with-manual-acks

    Ok(())
}

async fn update_persistent_subscription(client: &Client) -> eventstore::Result<()> {
    // #region update-persistent-subscription
    let options = PersistentSubscriptionOptions::default()
        .resolve_link_tos(true)
        .checkpoint_lower_bound(20);

    client
        .update_persistent_subscription("test-stream", "subscription-group", &options)
        .await?;
    // #endregion update-persistent-subscription

    Ok(())
}

async fn delete_persistent_subscription(client: &Client) -> eventstore::Result<()> {
    // #region delete-persistent-subscription
    client
        .delete_persistent_subscription("test-stream", "subscription-group", &Default::default())
        .await?;
    // #endregion delete-persistent-subscription

    Ok(())
}
