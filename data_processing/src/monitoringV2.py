import asyncio
from web3 import AsyncWeb3, WebSocketProvider
from web3 import Web3
import websockets
import tomllib
from pathlib import Path
import json

path = Path(__file__).resolve().parent.parent
config_path = path / "config.toml"

with open(config_path, "rb") as f:
    config = tomllib.load(f)

CSV_FILE = path / "data" / config["storage"]["data_storage_file_name"]
STORAGE_BUFFER_LENGTH = config["storage"]["storage_buffer_length"]
WSS_CEX = config["CEX"]["wss_url"]
COIN = config["CEX"]["coin"]
WSS_BLOCKCHAIN = config["blockchain"]["wss_url"]
POOL_ADDRESS = config["blockchain"]["pool_address"]
TOKEN0_DECIMAL = config["blockchain"]["token0_decimal"]
TOKEN1_DECIMAL = config["blockchain"]["token1_decimal"]
SWAP_TOPIC = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"


# SWAP PART ____________________________________________________________
async def listen_swaps(queue):
    async with AsyncWeb3(WebSocketProvider(WSS_BLOCKCHAIN)) as w3:
        subscription_id = await w3.eth.subscribe(
            "logs", {"address": POOL_ADDRESS, "topics": [SWAP_TOPIC]}
        )
        print("Subscribed to Swap events")

        async for message in w3.socket.process_subscriptions():
            result = message.get("result")
            if result:
                # print(result)
                block_number = result["blockNumber"]
                log_index = result["logIndex"]
                res = result["data"].hex()
                amount0_hex = res[0:64]
                amount1_hex = res[64:128]
                sqrtPriceX96_hex = res[128:192]
                liquidity_hex = res[192:256]

                price = round(
                    (int(sqrtPriceX96_hex, 16) / 2**96) ** 2
                    * 10 ** (TOKEN0_DECIMAL - TOKEN1_DECIMAL),
                    3,
                )
                event = {
                    "source": "uniswap",
                    "blockNumber": block_number,
                    "logIndex": log_index,
                    "amount0": hex_to_int256(amount0_hex),
                    "amount1": hex_to_int256(amount1_hex),
                    "sqrtPriceX96": int(sqrtPriceX96_hex, 16),
                    "price": price,
                    "liquidity": int(liquidity_hex, 16),
                }
                await queue.put(event)


def hex_to_int256(hex_str) -> int:
    value = int(hex_str, 16)
    if value >= 2**255:
        value -= 2**256
    return value


# HYPERLIQUID PART_________________________________________________


async def listen_hyperliquid(queue, last_bid, last_ask):
    async with websockets.connect(WSS_CEX) as ws:
        print("Connected to WebSocket Hyperliquid")

        subscribe_msg = {
            "method": "subscribe",
            "subscription": {
                "type": "l2Book",
                "coin": COIN,
            },
        }
        await ws.send(json.dumps(subscribe_msg))
        while True:
            message = await ws.recv()
            data = json.loads(message)

            try:
                time = data["data"]["time"]
                levels = data["data"]["levels"]
                bid = levels[0][0]["px"]
                ask = levels[1][0]["px"]
                if last_ask != ask or last_bid != bid:
                    event = {
                        "source": "hyperliquid",
                        "time": time,
                        "bid": bid,
                        "ask": ask,
                    }
                    last_ask = ask
                    last_bid = bid
                    await queue.put(event)

            except KeyError:
                pass
            except Exception as e:
                print(f"Fail with the error: {e}")
            except websockets.ConnectionClosed:
                print("Connexion closed")
                break


async def aggregator(queue, price_pool):
    rows = []
    last_swap = {}
    last_bid = 0
    last_ask = 0
    row_swap = [last_bid, last_ask, price_pool]
    while True:
        event = await queue.get()
        match event["source"]:
            case "hyperliquid":
                if last_swap:
                    # add last swap before HL
                    print("Nouvel event ajouté :", last_swap)
                    print(f"row : {row_swap}")
                    rows.append(row_swap)
                    last_swap = {}
                # add HL
                last_bid = event["bid"]
                last_ask = event["ask"]
                print("Nouvel event ajouté :", event)
                print(f"row : {[last_bid, last_ask, price_pool]}")
                rows.append([last_bid, last_ask, price_pool])
            case "uniswap":
                # first swap after HL
                if not last_swap:
                    last_swap = event
                    price_pool = last_swap["price"]
                    row_swap = [last_bid, last_ask, price_pool]

                # several swaps in same block
                elif last_swap["blockNumber"] == event["blockNumber"]:
                    print("Swap dans meme block")
                    if event["logIndex"] > last_swap["logIndex"]:
                        print(
                            f"lastSwap index: {last_swap['logIndex']}, new: {event['logIndex']}"
                        )
                        last_swap = event
                        price_pool = last_swap["price"]
                        row_swap = [last_bid, last_ask, price_pool]

                # new swap following a swap in another block
                else:
                    print("Nouvel event ajouté :", last_swap)
                    print(f"row : {[last_bid, last_ask, price_pool]}")
                    rows.append([last_bid, last_ask, price_pool])
                    last_swap = event
                    price_pool = last_swap["price"]
                    row_swap = [last_bid, last_ask, price_pool]
            case _:
                print("Unknow source")

        print("Taille stack :", len(rows))


async def main():
    last_ask = 0
    last_bid = 0
    pool_price = 0
    queue = asyncio.Queue()
    await asyncio.gather(
        listen_swaps(queue),
        listen_hyperliquid(queue, last_bid, last_ask),
        aggregator(queue, pool_price),
    )


if __name__ == "__main__":
    asyncio.run(main())
