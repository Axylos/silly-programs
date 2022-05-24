use serde::Deserialize;
use statsd::Client;
use std::fs;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[derive(Deserialize)]
struct Config {
    grafana_key: String,
    grafana_url: String,
    grafana_user: String,
    statsd_host: String,
    statsd_port: String,
}

#[tokio::main]
async fn main() -> Result<(), tracing_loki::Error> {
    let file_name = "/etc/silly/config.toml";
    let config_str = fs::read_to_string(file_name).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();

    let url_frag = format!(
        "https://{}:{}@{}",
        config.grafana_user, config.grafana_key, config.grafana_url
    );
    let url = Url::parse(&url_frag).unwrap();
    println!("{}", url);
    let (layer, task) = tracing_loki::layer(
        url,
        vec![("host".into(), "mine".into())].into_iter().collect(),
        vec![].into_iter().collect(),
    )?;

    let host = config.statsd_host;
    let port = config.statsd_port;
    let client = Client::new(format!("{}:{}", host, port), "loki-test").unwrap();

    client.incr("saw a dragon");
    client.incr("saw a dragon");
    client.incr("saw a human");

    client.incr("ate a wolverine");

    client.incr("completed a download by id");
    tracing_subscriber::registry().with(layer).init();

    tokio::spawn(task);

    tracing::info!(task = "did a thing", result = "success", "actual data");

    tracing::trace!(task = "trace a part", result = "maybe", "actual data");

    println!("Hello, world!");
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
