use async_graphql::{Subscription, Result};
use futures_util::stream::{Stream, StreamExt};
use std::time::Duration;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn ticks(&self) -> impl Stream<Item = i32> {
        let mut i = 0;
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| { i += 1; i })
    }
}
