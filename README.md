<p align="center">
  <h1 align="center">richamster_api</h1>
  <p align="center">
    <strong>Async Rust client for the Richamster cryptocurrency exchange API</strong>
  </p>
</p>

---

Type-safe, async wrapper for the [Richamster](https://richamster.com) cryptocurrency exchange REST API. Supports trading, account management, deposits, withdrawals, and real-time market data with multiple authentication strategies and secure credential handling.

> Originally created as a capstone project for Rust Bootcamp Summer 2023.

## Features

- **Async/Await** — built on `reqwest` for non-blocking I/O
- **Type-Safe API** — enum-based routing maps every endpoint to its URL and HTTP method at compile time
- **Multiple Auth Strategies** — JWT tokens, HMAC-SHA256 API key signing, or both combined
- **Secure Credentials** — secrets wrapped in `SecretBox` (zeroed on drop)
- **37 Supported Tokens** — BTC, ETH, USDT, SOL, ADA, DOT, XMR, DOGE, TON, and more
- **Builder Queries** — fluent filters for orders, transactions, and order books
- **Pagination** — built-in support for paginated responses
- **Comprehensive Errors** — unified `RichamsterError` enum via `thiserror`

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
richamster_api = { git = "https://github.com/shurikv/richamster_api" }
```

### Fetch Market Tickers (No Auth)

```rust
use richamster_api::richamster::exchange::Exchange;

let exchange = Exchange::new();
let tickers = exchange.ticker_list(None).await?;

for ticker in tickers {
    println!("{}: last price {}", ticker.pair, ticker.last);
}
```

### Place a Limit Order

```rust
use richamster_api::richamster::exchange::Exchange;
use richamster_api::models::exchange::NewOrder;
use richamster_api::models::common::OrderType;
use richamster_api::api::token::{CurrencyPair, Token};

let exchange = Exchange::with_keys("your_api_key".to_string(), "your_secret_key".to_string());

let order = NewOrder::new(
    "0.5".to_string(),
    "45000.00".to_string(),
    CurrencyPair::new(Token::BTC, Token::UAH),
    OrderType::Buying,
);

let result = exchange.create_order(order).await?;
println!("Order placed: {:?}", result);
```

### API Key Authentication

```rust
use richamster_api::richamster::exchange::Exchange;
use richamster_api::models::exchange::OrdersFilter;

let exchange = Exchange::with_keys("your_api_key".to_string(), "your_secret_key".to_string());
let orders = exchange.user_orders(OrdersFilter::new(None, None, None)).await?;
```

## Authentication

The library supports four authentication modes via `AuthState`:

| Mode | Use Case |
|------|----------|
| `Unauthorized` | Public endpoints (tickers, markets, currencies) |
| `JwtTokenAuth` | Session-based access after login |
| `ApiSecretKeyAuth` | Stateless API access with HMAC-SHA256 request signing |
| `JwtTokenWithApiSecretKeyAuth` | Combined — maximum access |

> **Note:** The Richamster exchange currently only supports API key authentication. JWT-based methods may be available in the future.

API key requests are signed using HMAC-SHA256: the request body is hashed with your secret key and sent in the `Signature` header alongside the `Api-Key` header.

## API Coverage

### Exchange

| Method | Description |
|--------|-------------|
| `ticker_list` | Market ticker data for all or specific pairs |
| `markets_list` | List all available markets |
| `currencies_list` | Currency details, optionally filtered by token |
| `restrictions_list` | Trading restrictions per currency pair |
| `order_book` | Buy/sell order book with filters |
| `user_orders` | Current user's open orders |
| `orders_history` | Paginated order history |
| `create_order` | Place a new limit order |
| `destroy_user_order` | Cancel an existing order |
| `calculate_market_order` | Estimate market order execution |
| `execute_market_order` | Execute a market order |
| `favourites_pair_toggle` | Toggle a pair as favourite |

### User

| Method | Description |
|--------|-------------|
| `balances` | Account balances (with USDT/BTC equivalents) |
| `detail_info` | User profile information |
| `orders` | Paginated order history with filters |
| `transactions_list` | Transaction history with filters |
| `transfer` | Transfer funds between accounts |

### Withdraw & Deposit

| Method | Description |
|--------|-------------|
| `withdraw_info` | Withdrawal fees and available channels |
| `withdraw` | Execute a withdrawal |
| `replenish_info` | Get blockchain deposit address |
| `replenish_channels_info` | Available deposit channels for a currency |
| `replenish_p2p` | P2P deposit |

### Auth

| Method | Description |
|--------|-------------|
| `login` | Email/password authentication |
| `register_user` | Create a new account |
| `two_factor_login` | Complete 2FA verification |
| `refresh_token` | Refresh an expired JWT |

## Supported Tokens

```
AAVE   ADA    BAT    BOX    BTC    CRO    DASH   DOGE   DOT
ETH    FSH    HCK    KRB    KUB    LINK   LTC    NFT    POL
RCH    SHIB   SOL    TAL    TLR    TON    TRX    UAHT   UAH
UNI    USDC   USDT   VQR    WAVES  WLD    XAUT   XMR
```

Currency pairs use `"BTC/UAH"` format with case-insensitive parsing.

## Project Structure

```
src/
├── api/          # Enum-based routing & token definitions
│   ├── mod.rs    # Api enum, RequestPath trait, endpoint URLs
│   └── token.rs  # Token enum (37 currencies), CurrencyPair
├── errors/       # Unified RichamsterError (thiserror)
├── models/       # Request/response DTOs
│   ├── auth.rs       # Login, Register, OTP, TokenData
│   ├── common.rs     # Currency, OrderType, TransactionStatus
│   ├── exchange.rs   # Ticker, Order, OrderBook, Market
│   ├── user.rs       # Balance, UserDetail, Transactions
│   ├── withdraw.rs   # WithdrawData, WithdrawInfo
│   ├── replenish.rs  # ReplenishInfo, P2PReplenish
│   └── feedback.rs   # Messenger, ContactUs
├── richamster/   # Business logic & API operations
│   ├── common.rs     # AuthState, JwtToken, ApiKey, SecretKey
│   ├── auth.rs       # Login, register, 2FA, token refresh
│   ├── exchange.rs   # Trading, orders, market data
│   ├── user.rs       # Balances, profile, transactions
│   ├── withdraw.rs   # Withdrawals
│   ├── replenish.rs  # Deposits
│   └── feedback.rs   # Contact/messenger info
└── lib.rs        # Public module exports

example/          # Interactive CLI demo application
```
