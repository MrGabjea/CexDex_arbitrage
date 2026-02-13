use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use serde_json::json;
use super::LiveEvent;
use tokio::sync::mpsc::Sender;

pub async fn hyperliquid_feed(
    ws_url: &str,
    coin: &str,
    sender: Sender<LiveEvent>,
) ->anyhow::Result<()> {

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
                let event = LiveEvent::Hyperliquid {
                    coin: coin.to_string(),
                    raw_message: text,
                };

                if sender.send(event).await.is_err() {
                    break;
                    }
            },
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
