import asyncio
from web3 import AsyncWeb3, WebSocketProvider
from web3 import Web3

"""
path = Path(__file__).resolve().parent.parent
config_path = path / "config.toml"

with open(config_path, "rb") as f:
    config = tomllib.load(f)

CSV_FILE = path / "data" / config["storage"]["data_storage_file_name"]
storage_buffer_length = config["storage"]["storage_buffer_length"]
WS_URL = config["CEX"]["ws_url"]
coin = config["CEX"]["coin"]


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
        last_ask = 0
        last_bid = 0
        while True:
            try:
                message = await ws.recv()
                data = json.loads(message)
                try:
                    time = data["data"]["time"]
                    list_prix = data["data"]["levels"]
                    bid = list_prix[0][0]
                    ask = list_prix[1][0]
                    if last_ask != ask["px"] or last_bid != bid["px"]:
                        sqrt_price_x96, *_ = pool.functions.slot0().call()
                        price_pool = round(
                            (sqrt_price_x96 / 2**96) ** 2
                            * 10 ** (token0_decimal - token1_decimal),
                            2,
                        )
                        print(f"Prix de la pool: {price_pool}")
                        print(f"bid : {bid['px']}")
                        print(f"ask : { ask['px']}")
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
"""

WSS_ADDRESS = "wss://arbitrum.drpc.org"
POOL_ADDRESS = "0xC6962004f452bE9203591991D15f6b388e09E8D0"
SWAP_TOPIC = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"


async def listen_swaps():
    async with AsyncWeb3(WebSocketProvider(WSS_ADDRESS)) as w3:
        # on s’abonne aux logs filtrés par adresse et topic
        subscription_id = await w3.eth.subscribe(
            "logs", {"address": POOL_ADDRESS, "topics": [SWAP_TOPIC]}
        )
        print("Subscribed to Swap events")

        async for message in w3.socket.process_subscriptions():
            result = message.get("result")
            if result:
                print("Received log, data", result)
                res = result["data"].hex()
                amount0_hex = res[0:64]
                amount1_hex = res[64:128]
                sqrtPriceX96_hex = res[128:192]
                liquidity_hex = res[192:256]

                res_dic = {
                    "amount0": hex_to_int256(amount0_hex),
                    "amount1": hex_to_int256(amount1_hex),
                    "sqrtPriceX96": int(sqrtPriceX96_hex, 16),
                    "liquidity": int(liquidity_hex, 16),
                }

                print(res_dic)


def hex_to_int256(hex_str) -> int:
    value = int(hex_str, 16)
    if value >= 2**255:
        value -= 2**256
    return value


asyncio.run(listen_swaps())
