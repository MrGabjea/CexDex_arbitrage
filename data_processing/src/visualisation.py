import pandas as pd
import tomllib
from pathlib import Path
import matplotlib.pyplot as plt

# load config
path = Path(__file__).resolve().parent.parent
config_path = path / "config.toml"

with open(config_path, "rb") as f:
    config = tomllib.load(f)

CSV_FILE = path / "data" / config["storage"]["data_storage_file_name"]
n_start = config["visualisation"]["n_start"]
n_end = config["visualisation"]["n_end"]

# load data
df = pd.read_csv(CSV_FILE, header=None)
columns = ["timestamp", "bid", "ask", "price_pool"]
df.columns = columns

df["spread_bid"] = df["price_pool"] - df["bid"]
df["spread_ask"] = df["ask"] - df["price_pool"]

df["spread_taker"] = (
    100
    * (-df[["spread_bid", "spread_ask"]].min(axis=1).clip(upper=0))
    / df["price_pool"]
)

plt.figure()
plt.plot(df["price_pool"][n_start:n_end], label="price_pool")
plt.plot(df["bid"][n_start:n_end], label="bid")
plt.plot(df["ask"][n_start:n_end], label="ask")
plt.title("Evolution of price")
plt.grid()
plt.legend()

plt.figure()
plt.plot(df["spread_taker"][n_start:n_end], label="spread taker (%)")
plt.title("Spread between taker orders ")
plt.grid()
plt.legend()

plt.show()
