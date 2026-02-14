use super::LiveEvent;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use serde_json::json;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn hyperliquid_feed(
    ws_url: &str,
    coin: &str,
    sender: Sender<LiveEvent>,
) -> anyhow::Result<()> {
    // Connexion TLS sécurisée
    let (ws_stream, _) = connect_async(ws_url)
        .await
        .expect("Échec de la connexion à la websocket");

    println!("Connected to {}", ws_url);

    let (mut write, mut read) = ws_stream.split();

    // Préparer le message de subscription
    let subscribe_msg = json!({
        "method": "subscribe",
        "subscription": {
            "type": "l2Book",
            "coin": coin
        }
    });

    write
        .send(Message::Text(subscribe_msg.to_string()))
        .await
        .expect("Échec de l'envoi du message de subscription");

    println!("Subscription Message sent : {}", subscribe_msg);

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let parsed_text: Result<Value, _> = serde_json::from_str(&text);
                match parsed_text {
                    Ok(res) => {
                        if let Some(time_u64) = res
                            .get("data")
                            .and_then(|d| d.get("time"))
                            .and_then(|t| t.as_u64())
                            && let Some(Value::Array(l2_book)) =
                                res.get("data").and_then(|k| k.get("levels"))
                        {
                            let event = LiveEvent::Hyperliquid {
                                timestamp: time_u64,
                                levels: l2_book.clone(),
                            };

                            if sender.send(event).await.is_err() {
                                break;
                            }
                        }
                    }
                    _ => {
                        println!("Pas possible de parse")
                    }
                }
                // let event = LiveEvent::Hyperliquid {
                //     coin: coin.to_string(),
                //     raw_message: serde_json::from_str(&text),
                // };

                // if sender.send(event).await.is_err() {
                //     break;
                //     }
            }
            Ok(Message::Close(_)) => {
                println!("Connexion fermée par le serveur");
                break;
            }
            Err(e) => {
                eprintln!("Erreur websocket : {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
