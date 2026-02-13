import asyncio
from web3 import AsyncWeb3, WebSocketProvider

WSS_ADDRESS = "wss://arbitrum.drpc.org"
POOL_ADDRESS = "0xC6962004f452bE9203591991D15f6b388e09E8D0"
SWAP_TOPIC = "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"


async def listen_swaps():
    async with AsyncWeb3(WebSocketProvider(WSS_ADDRESS)) as w3:
        # on s’abonne aux logs filtrés par adresse et topic
        await w3.eth.subscribe(
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
