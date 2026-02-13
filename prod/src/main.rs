mod live_feed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    live_feed::livefeed::start_livefeed().await
}
