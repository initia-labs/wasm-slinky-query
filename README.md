# Wasm Slinky Query

CosmWasm contract to handle skip oracle queries.

## InstantiateMsg

Initialize Slinky contract

```json
{}
```

## QueryMsg

### `get_price`

Get oracle price

```json
{
  "get_price": {
    "base": "BITCOIN",
    "quote": "USD"
  }
}
```

Response type

```json
{
  "price": {
    "price": "6942640000000",
    "block_timestamp": "1712446136721825744", // uinx timestamp in nano second
    "block_height": 561283
  },
  "nonce": 12243,
  "decimals": 8,
  "id": 0
}
```

### `get_prices`

Get oracle prices

```json
{
  "get_prices": {
    "pair_ids": ["BITCOIN/USD"] // {Base}/{Quote}
  }
}
```

Response type

```json
{
  "prices": [
    {
      "price": {
        "price": "6942640000000",
        "block_timestamp": "1712446136721825744",
        "block_height": 561283
      },
      "nonce": 12243,
      "decimals": 8,
      "id": 0
    },
    ...
  ]
}
```

### `get_all_currency_pairs`

Get all currency pairs that can get oracle price

```json
{
  "get_all_currency_pairs": {}
}
```

Response type

```json
{
  "currency_pairs": [
    {
      "Base": "BITCOIN",
      "Quote": "USD"
    },
    ...
  ]
}
```
