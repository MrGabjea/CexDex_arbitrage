import asyncio
import json
import websockets
from web3 import Web3
import tomllib
from pathlib import Path
import csv

path = Path(__file__).resolve().parent.parent
config_path = path / "config.toml"

with open(config_path, "rb") as f:
    config = tomllib.load(f)

CSV_FILE = path / "data" / config["storage"]["data_storage_file_name"]
WS_URL = config["CEX"]["ws_url"]
coin = config["CEX"]["coin"]
rpc_url = config["blockchain"]["rpc_url"]
pool_address = config["blockchain"]["pool_address"]
token0_decimal = config["blockchain"]["token0_decimal"]
token1_decimal = config["blockchain"]["token1_decimal"]

UNISWAP_V3_POOL_ABI = [
    {
        "inputs": [],
        "name": "slot0",
        "outputs": [
            {"name": "sqrtPriceX96", "type": "uint160"},
            {"name": "tick", "type": "int24"},
            {"name": "observationIndex", "type": "uint16"},
            {"name": "observationCardinality", "type": "uint16"},
            {"name": "observationCardinalityNext", "type": "uint16"},
            {"name": "feeProtocol", "type": "uint8"},
            {"name": "unlocked", "type": "bool"},
        ],
        "stateMutability": "view",
        "type": "function",
    },
    {
        "inputs": [],
        "name": "token0",
        "outputs": [{"name": "", "type": "address"}],
        "stateMutability": "view",
        "type": "function",
    },
    {
        "inputs": [],
        "name": "token1",
        "outputs": [{"name": "", "type": "address"}],
        "stateMutability": "view",
        "type": "function",
    },
]

w3 = Web3(Web3.HTTPProvider(rpc_url))
pool = w3.eth.contract(
    address=Web3.to_checksum_address(pool_address), abi=UNISWAP_V3_POOL_ABI
)


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
        rows = []
        last_ask = 0
        last_bid = 0
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
                        sqrt_price_x96, *_ = pool.functions.slot0().call()
                        price_pool = round(
                            (sqrt_price_x96 / 2**96) ** 2
                            * 10 ** (token0_decimal - token1_decimal),
                            2,
                        )
                        # print(f"Prix de la pool: {price_pool}")
                        # print(f"bid : {bid['px']}")
                        # print(f"ask : { ask['px']}")
                        row = [
                            time,
                            bid["px"],
                            ask["px"],
                            price_pool
                        ]
                        last_ask = ask["px"]
                        last_bid = bid["px"]
                        rows.append(row)
                        if len(rows) > 5:
                            with open(CSV_FILE, "a", newline="", encoding="utf-8") as f:
                                writer = csv.writer(f)
                                writer.writerows(rows)
                            print(
                                f"{len(rows)} rows has been successfully added to the dataset ! "
                            )
                            rows = []
                except KeyError:
                    ()
                except Exception as e:
                    print(f"Fail with the error: {e}")
            except websockets.ConnectionClosed:
                print("Connexion closed")
                break


asyncio.run(main())
