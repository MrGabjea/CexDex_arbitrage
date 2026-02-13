import asyncio
import json
import websockets
from web3 import Web3

WS_URL = "wss://api.hyperliquid.xyz/ws"
coin = "ETH"

async def main():
    async with websockets.connect(WS_URL) as ws:
        print("Connected to WebSocket Hyperliquid")

        subscribe_msg = {
            "method": "subscribe",
            "subscription": {
                "type": "l2Book",
                "coin": coin,
            },
        }
        await ws.send(json.dumps(subscribe_msg))
        last_ask=0
        last_bid=0
        while True:
            try:
                message = await ws.recv()
                # print("message recu")
                data = json.loads(message)
                try:
                    time = data["data"]["time"]
                    list_prix = data["data"]["levels"]
                    bid = list_prix[0][0]
                    ask = list_prix[1][0]
                    if last_ask != ask["px"] or last_bid != bid["px"]:
                        print("New bid-ask:")
                        print(f"bid : {bid['px']}")
                        print(f"ask : { ask['px']}\n")
                        last_ask = ask["px"]
                        last_bid = bid["px"]
                except KeyError:
                    ()
                except Exception as e:
                    print(f"Fail with the error: {e}")
            except websockets.ConnectionClosed:
                print("Connexion closed")
                break


asyncio.run(main())
