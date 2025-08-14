# API Endpoints Reference

This document provides a comprehensive reference of all REST API endpoints and WebSocket connections used by the axiomtrade-rs SDK. The Axiom Trade platform uses multiple redundant API servers for load balancing and high availability.

## Base URLs and Load Balancing

### Primary API Servers
```
https://api2.axiom.trade
https://api3.axiom.trade
https://api6.axiom.trade
https://api7.axiom.trade
https://api8.axiom.trade
https://api9.axiom.trade
https://api10.axiom.trade
```

The SDK automatically rotates between these endpoints for load balancing and fault tolerance. Authentication and most trading operations work across all servers.

### Specialized Endpoints
- **Main Domain**: `https://axiom.trade` - Used for certain portfolio operations
- **WebSocket Clusters**: Various regional endpoints for real-time data
- **External Services**: MEV protection and infrastructure monitoring

## Authentication Endpoints

### Login Flow

#### POST `/login-password-v2`
**Description**: First step of two-factor authentication process

**Authentication**: None required

**Request Body**:
```json
{
  "email": "user@example.com",
  "b64_password": "base64_encoded_pbkdf2_hash"
}
```

**Response**:
```json
{
  "otp_jwt_token": "jwt_token_for_otp_step"
}
```

**Rate Limiting**: 5 requests per minute per IP

---

#### POST `/login-otp`
**Description**: Second step of authentication using OTP code

**Authentication**: Requires `auth-otp-login-token` cookie from step 1

**Headers**:
```
Cookie: auth-otp-login-token={otp_jwt_token}
```

**Request Body**:
```json
{
  "code": "123456",
  "email": "user@example.com",
  "b64_password": "base64_encoded_pbkdf2_hash"
}
```

**Response**:
```json
{
  "access_token": "jwt_access_token",
  "refresh_token": "jwt_refresh_token",
  "user": {
    "id": "user_id",
    "email": "user@example.com"
  },
  "org_id": "turnkey_organization_id",
  "user_id": "turnkey_user_id",
  "client_secret": "turnkey_client_secret"
}
```

**Cookies Set**:
- `auth-access-token`: JWT access token (15 minutes)
- `auth-refresh-token`: JWT refresh token (30 days)

---

#### POST `/refresh-access-token`
**Description**: Refresh expired access token using refresh token

**Authentication**: Requires `auth-refresh-token` cookie

**Headers**:
```
Cookie: auth-refresh-token={refresh_token}
```

**Response**:
```json
{
  "access_token": "new_jwt_access_token"
}
```

**Cookies Set**:
- `auth-access-token`: New JWT access token (15 minutes)

## Trading Endpoints

All trading endpoints require authentication via `auth-access-token` cookie.

### POST `/batched-send-tx-v2`
**Description**: Execute buy, sell, or swap orders

**Authentication**: Required

**Headers**:
```
Cookie: auth-access-token={access_token}
Content-Type: application/json
```

**Request Body (Buy Order)**:
```json
{
  "token_mint": "token_mint_address",
  "amount_sol": 1.5,
  "slippage_percent": 5.0,
  "priority_fee": 5000
}
```

**Request Body (Sell Order)**:
```json
{
  "token_mint": "token_mint_address",
  "amount_tokens": 1000.0,
  "slippage_percent": 5.0,
  "priority_fee": 5000
}
```

**Request Body (Swap Order)**:
```json
{
  "from_mint": "source_token_mint",
  "to_mint": "destination_token_mint",
  "amount": 1000.0,
  "slippage_percent": 5.0,
  "priority_fee": 5000
}
```

**Response**:
```json
{
  "status": "Success",
  "signature": "transaction_signature",
  "transaction_id": "tx_id"
}
```

**Error Responses**:
- `400`: Invalid parameters, insufficient balance, slippage exceeded
- `401`: Authentication required
- `429`: Rate limit exceeded

---

### POST `/quote`
**Description**: Get pricing quote for token swaps

**Authentication**: Required

**Request Body**:
```json
{
  "input_mint": "input_token_mint",
  "output_mint": "output_token_mint",
  "amount": 1000.0,
  "slippage_percent": 5.0
}
```

**Response**:
```json
{
  "input_amount": 1000.0,
  "output_amount": 950.0,
  "price_impact": 2.5,
  "estimated_gas": 5000
}
```

---

### POST `/simulate`
**Description**: Simulate transaction before execution

**Authentication**: Required

**Request Body**:
```json
{
  "transaction": "base64_encoded_transaction"
}
```

**Response**:
```json
{
  "success": true,
  "estimated_gas": 5000,
  "error": null
}
```

## Portfolio Endpoints

### POST `/batched-sol-balance` (Main Domain Only)
**Description**: Get SOL and token balances for multiple wallets

**Base URL**: `https://axiom.trade/api` (Main domain only)

**Authentication**: Required

**Request Body**:
```json
{
  "public_keys": [
    "wallet_address_1",
    "wallet_address_2"
  ]
}
```

**Response**:
```json
{
  "wallet_address_1": {
    "sol_balance": 2.5,
    "tokens": [
      {
        "mint_address": "token_mint",
        "balance": 1000.0,
        "balance_usd": 150.0
      }
    ],
    "total_value_usd": 400.0
  }
}
```

---

### POST `/portfolio-v5`
**Description**: Get comprehensive portfolio summary

**Authentication**: Required

**Request Body**:
```json
{
  "walletAddressRaw": "address1,address2",
  "isOtherWallet": false,
  "totalSolBalance": 5.0,
  "tokenAddressToAmountMap": {},
  "timeOffset": -480
}
```

**Note**: Wallet addresses must be sorted alphabetically before joining with commas.

**Response**:
```json
{
  "total_value_usd": 1500.0,
  "pnl_24h": 50.0,
  "positions": []
}
```

## Market Data Endpoints

Market data endpoints use specialized API servers (typically `api6.axiom.trade`).

### GET `/meme-trending`
**Description**: Get trending meme tokens

**Authentication**: Required

**Query Parameters**:
- `timePeriod`: `1h`, `24h`, `7d`, `30d`

**Example**: `GET /meme-trending?timePeriod=24h`

**Response**:
```json
[
  {
    "tokenAddress": "token_mint_address",
    "tokenTicker": "BONK",
    "tokenName": "Bonk Token",
    "priceUsd": 0.00001,
    "marketCapPercentChange": 15.5,
    "volumeSol": 1000.0,
    "marketCapSol": 50000.0,
    "top10Holders": 25.5,
    "tokenImage": "https://image_url"
  }
]
```

---

### GET `/token-analysis`
**Description**: Get detailed token information and creator analysis

**Authentication**: Required

**Query Parameters**:
- `tokenTicker`: Token symbol (e.g., "BONK", "SOL")

**Example**: `GET /token-analysis?tokenTicker=BONK`

**Response**:
```json
{
  "symbol": "BONK",
  "name": "Bonk Token",
  "mint_address": "token_mint_address",
  "price_usd": 0.00001,
  "market_cap": 50000000,
  "volume_24h": 1000000,
  "creator_analysis": {}
}
```

---

### GET `/clipboard-pair-info`
**Description**: Get quick token info by mint or pair address

**Authentication**: Required

**Query Parameters**:
- `address`: Token mint address or pair address

**Example**: `GET /clipboard-pair-info?address={mint_address}`

---

### GET `/price/{token_mint}`
**Description**: Get current price data for a token

**Authentication**: Required

**Response**:
```json
{
  "price_usd": 0.00001,
  "price_sol": 0.000001,
  "price_change_24h": 5.5,
  "volume_24h": 1000.0,
  "timestamp": 1640995200
}
```

---

### GET `/price-feed/{token_mint}`
**Description**: Get historical price feed

**Authentication**: Required

**Query Parameters**:
- `period`: `1h`, `24h`, `7d`, `30d`

**Response**:
```json
{
  "prices": [
    {
      "timestamp": 1640995200,
      "price_usd": 0.00001,
      "volume": 1000.0
    }
  ]
}
```

---

### GET `/chart/{token_mint}`
**Description**: Get chart data with candles

**Authentication**: Required

**Query Parameters**:
- `timeframe`: `1m`, `5m`, `15m`, `1h`, `4h`, `1d`, `1w`
- `limit`: Maximum number of candles (optional)

**Response**:
```json
{
  "candles": [
    {
      "timestamp": 1640995200,
      "open": 0.00001,
      "high": 0.000012,
      "low": 0.000009,
      "close": 0.000011,
      "volume": 1000.0
    }
  ]
}
```

---

### GET `/search-v3`
**Description**: Search for tokens by name or symbol

**Authentication**: Required

**Query Parameters**:
- `searchQuery`: Search term (URL encoded)
- `limit`: Maximum results (optional)

**Response**:
```json
[
  {
    "symbol": "BONK",
    "name": "Bonk Token",
    "mint_address": "token_mint_address",
    "price_usd": 0.00001,
    "logo_uri": "https://image_url"
  }
]
```

---

### POST `/batch-prices`
**Description**: Get batch price data for multiple tokens

**Authentication**: Required

**Request Body**:
```json
{
  "mints": [
    "token_mint_1",
    "token_mint_2"
  ]
}
```

**Response**:
```json
[
  {
    "mint_address": "token_mint_1",
    "price_usd": 0.00001,
    "price_change_24h": 5.5
  }
]
```

## Social Trading Endpoints

Social features use `api8.axiom.trade` base URL.

### GET `/tracked-wallets-v2`
**Description**: Get user's tracked wallets

**Authentication**: Required

**Response**:
```json
[
  {
    "address": "wallet_address",
    "name": "Wallet Name",
    "added_at": "2024-01-01T00:00:00Z",
    "performance": {
      "pnl_24h": 150.0,
      "win_rate": 75.5
    }
  }
]
```

---

### POST `/tracked-wallets-v2`
**Description**: Add or remove tracked wallets

**Authentication**: Required

**Request Body (Add)**:
```json
{
  "address": "wallet_address",
  "name": "Optional Name",
  "action": "add"
}
```

**Request Body (Remove)**:
```json
{
  "address": "wallet_address",
  "action": "remove"
}
```

---

### POST `/tracked-wallet-transactions-v2`
**Description**: Get transactions from tracked wallets

**Authentication**: Required

**Request Body**:
```json
{
  "wallet_addresses": ["address1", "address2"],
  "limit": 50,
  "offset": 0,
  "time_range": "24h"
}
```

**Response**:
```json
[
  {
    "signature": "tx_signature",
    "wallet_address": "wallet_address",
    "token_mint": "token_mint",
    "action": "buy",
    "amount_sol": 1.0,
    "timestamp": "2024-01-01T00:00:00Z"
  }
]
```

---

### GET `/watchlist`
**Description**: Get user's token watchlist

**Authentication**: Required

**Response**:
```json
[
  {
    "token_address": "token_mint",
    "symbol": "BONK",
    "added_at": "2024-01-01T00:00:00Z",
    "current_price": 0.00001
  }
]
```

---

### POST `/watchlist`
**Description**: Add or remove tokens from watchlist

**Authentication**: Required

**Request Body (Add)**:
```json
{
  "tokenAddress": "token_mint",
  "symbol": "BONK",
  "action": "add"
}
```

**Request Body (Remove)**:
```json
{
  "tokenAddress": "token_mint",
  "action": "remove"
}
```

---

### GET `/twitter-feed-new-2`
**Description**: Get Twitter/X feed with trading content

**Authentication**: Required

**Query Parameters**:
- `includeTruthSocial`: `true` or `false`

**Response**:
```json
[
  {
    "id": "tweet_id",
    "author": "username",
    "content": "tweet content",
    "timestamp": "2024-01-01T00:00:00Z",
    "mentions": ["$BONK"],
    "engagement": {
      "likes": 100,
      "retweets": 50
    }
  }
]
```

---

### GET `/twitter-settings`
**Description**: Get user's Twitter feed settings

**Authentication**: Required

**Response**:
```json
{
  "enabled": true,
  "keywords": ["solana", "meme"],
  "min_followers": 1000,
  "include_truth_social": false
}
```

## Notifications Endpoints

### GET `/get-notifications`
**Description**: Get user notifications

**Authentication**: Required

**Response**:
```json
[
  {
    "id": "notification_id",
    "type": "price_alert",
    "title": "Price Alert",
    "message": "BONK reached $0.00002",
    "timestamp": "2024-01-01T00:00:00Z",
    "read": false
  }
]
```

---

### GET `/get-announcement`
**Description**: Get system announcements

**Authentication**: Required

**Response**:
```json
[
  {
    "id": "announcement_id",
    "title": "System Maintenance",
    "content": "Scheduled maintenance...",
    "timestamp": "2024-01-01T00:00:00Z",
    "priority": "medium"
  }
]
```

---

### POST `/notifications/{id}/read`
**Description**: Mark notification as read

**Authentication**: Required

---

### POST `/notifications/read-all`
**Description**: Mark all notifications as read

**Authentication**: Required

---

### POST `/alerts/price`
**Description**: Create price alert

**Authentication**: Required

**Request Body**:
```json
{
  "token_address": "token_mint",
  "condition": "above",
  "price": 0.00002,
  "notification_method": "email"
}
```

**Response**:
```json
{
  "alertId": "alert_id"
}
```

---

### DELETE `/alerts/price/{alert_id}`
**Description**: Delete price alert

**Authentication**: Required

---

### POST `/alerts/wallet`
**Description**: Create wallet activity alert

**Authentication**: Required

**Request Body**:
```json
{
  "wallet_address": "wallet_address",
  "min_amount_sol": 1.0,
  "notification_method": "email"
}
```

---

### GET `/notifications/settings`
**Description**: Get notification settings

**Authentication**: Required

---

### PUT `/notifications/settings`
**Description**: Update notification settings

**Authentication**: Required

## External Integrations

### Hyperliquid API
**Base URL**: `https://api.hyperliquid.xyz`

**Authentication**: None (public API)

#### POST `/info`
**Description**: Get various Hyperliquid data

**Request Body Examples**:

**Clearinghouse State**:
```json
{
  "type": "clearinghouseState",
  "user": "ethereum_address"
}
```

**Market Metadata**:
```json
{
  "type": "meta"
}
```

**All Mid Prices**:
```json
{
  "type": "allMids"
}
```

**Open Orders**:
```json
{
  "type": "openOrders",
  "user": "ethereum_address"
}
```

**User Fills**:
```json
{
  "type": "userFills",
  "user": "ethereum_address"
}
```

**L2 Orderbook**:
```json
{
  "type": "l2Book",
  "coin": "BTC"
}
```

**Recent Trades**:
```json
{
  "type": "recentTrades",
  "coin": "BTC"
}
```

**24hr Stats**:
```json
{
  "type": "24hrStats"
}
```

**User Funding**:
```json
{
  "type": "userFunding",
  "user": "ethereum_address",
  "startTime": 1640995200000,
  "endTime": 1641081600000
}
```

### Turnkey API
**Base URL**: `https://api.turnkey.com`

**Authentication**: P256 signature in X-Stamp header

#### POST `/public/v1/query/whoami`
**Description**: Get user identity information

**Headers**:
```
Content-Type: text/plain;charset=UTF-8
X-Stamp: base64_encoded_signature
x-client-version: @turnkey/sdk-server@1.7.3
```

**Request Body**:
```json
{
  "organization_id": "turnkey_org_id"
}
```

---

#### POST `/public/v1/query/get_api_keys`
**Description**: Get API keys for a user

**Headers**: Same as whoami

**Request Body**:
```json
{
  "user_id": "turnkey_user_id",
  "organization_id": "turnkey_org_id"
}
```

---

#### POST `/public/v1/submit/create_read_write_session`
**Description**: Create read/write session

**Headers**: Same as whoami

**Request Body**:
```json
{
  "parameters": {
    "api_key_name": "session_key_name",
    "target_public_key": "p256_public_key",
    "user_id": "turnkey_user_id",
    "expiration_seconds": "2592000"
  },
  "organization_id": "turnkey_org_id",
  "timestamp_ms": "1640995200000",
  "activity_type": "ACTIVITY_TYPE_CREATE_READ_WRITE_SESSION_V2"
}
```

---

#### GET `/public/v1/health`
**Description**: Health check endpoint

**Authentication**: None

## WebSocket Endpoints

### Regional WebSocket Clusters

**Connection URLs**:
```
wss://socket8.axiom.trade/          (US West, Global)
wss://cluster3.axiom.trade/         (US Central)
wss://cluster5.axiom.trade/         (US East)
wss://cluster6.axiom.trade/         (EU West)
wss://cluster2.axiom.trade/         (EU Central)
wss://cluster8.axiom.trade/         (EU East)
wss://cluster4.axiom.trade/         (Asia)
wss://cluster7.axiom.trade/         (Australia)
wss://cluster9.axiom.trade/         (Global)
```

### Connection Headers
```
Cookie: auth-access-token={access_token}; auth-refresh-token={refresh_token}
Origin: https://axiom.trade
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36
```

### Subscription Messages

#### Subscribe to New Token Pairs
```json
{
  "action": "join",
  "room": "new_pairs"
}
```

**Response Format**:
```json
{
  "room": "new_pairs",
  "content": {
    "token_address": "token_mint",
    "token_ticker": "SYMBOL",
    "token_name": "Token Name",
    "initial_liquidity_sol": 10.0,
    "supply": 1000000000
  }
}
```

#### Subscribe to Token Price Updates
```json
{
  "action": "join",
  "room": "token_mint_address"
}
```

#### Subscribe to Wallet Transactions
```json
{
  "action": "join",
  "room": "v:wallet_address"
}
```

### Message Types

**Connection Established**:
```json
{
  "type": "connected",
  "session_id": "session_id"
}
```

**Connection Lost**:
```json
{
  "type": "disconnected",
  "reason": "token_expired"
}
```

**Market Update**:
```json
{
  "type": "market_update",
  "data": {
    "token_mint": "token_address",
    "symbol": "BONK",
    "price_usd": 0.00001,
    "price_change_24h": 5.5,
    "volume_24h": 1000.0,
    "timestamp": 1640995200
  }
}
```

## Infrastructure Monitoring

### Lighthouse Service
**URL**: `https://api8.axiom.trade/lighthouse`
**Method**: GET
**Authentication**: None

### MEV Protection Services

#### 0slot Network
```
https://la1.0slot.trade/health      (Los Angeles)
https://ny3.0slot.trade/health      (New York)
https://de1.0slot.trade/health      (Germany)
https://ams1.0slot.trade/health     (Amsterdam)
https://jp1.0slot.trade/health      (Japan)
```

#### Nozomi Temporal Network
```
https://lax1.secure.nozomi.temporal.xyz/ping    (LAX)
https://ewr1.secure.nozomi.temporal.xyz/ping    (EWR)
https://ams1.secure.nozomi.temporal.xyz/ping    (AMS)
https://fra2.secure.nozomi.temporal.xyz/ping    (FRA)
https://ash1.secure.nozomi.temporal.xyz/ping    (ASH)
https://sgp1.secure.nozomi.temporal.xyz/ping    (SGP)
https://tyo1.secure.nozomi.temporal.xyz/ping    (TYO)
https://pit1.secure.nozomi.temporal.xyz/ping    (PIT)
https://nozomi.temporal.xyz/ping                (Main)
```

#### External MEV Protection
**URL**: `https://tx.axiomext.net/ping`

#### Jito Block Engine
```
https://slc.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts      (Salt Lake City)
https://london.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts   (London)
https://frankfurt.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts (Frankfurt)
https://ny.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts       (New York)
https://tokyo.mainnet.block-engine.jito.wtf/api/v1/getTipAccounts    (Tokyo)
```

#### Astralane Gateway
```
https://axiom-fra.gateway.astralane.io/gethealth?api-key={api_key}   (Frankfurt)
https://axiom-ca.gateway.astralane.io/gethealth?api-key={api_key}    (Canada)
```

**API Key**: `AxiomozyNSTbBlP88VY35BvSdDVS3du1be8Q1VMmconPgpWFVWnpmfnpUrhRj97F`

#### Arbitrum RPC
**URL**: `https://arb1.arbitrum.io/rpc`
**Method**: POST
**Body**:
```json
{
  "jsonrpc": "2.0",
  "method": "eth_blockNumber",
  "params": [],
  "id": 1
}
```

## Rate Limiting

### Global Limits
- **Authentication**: 5 requests per minute per IP
- **Trading**: 10 requests per minute per user
- **Market Data**: 100 requests per minute per user
- **WebSocket**: 1 connection per user per region

### Error Responses
**429 Too Many Requests**:
```json
{
  "error": "Rate limit exceeded",
  "retry_after": 60,
  "limit": 100,
  "remaining": 0
}
```

## Error Handling

### HTTP Status Codes
- `200`: Success
- `400`: Bad Request (invalid parameters)
- `401`: Unauthorized (invalid or expired token)
- `403`: Forbidden (insufficient permissions)
- `404`: Not Found (resource not found)
- `429`: Too Many Requests (rate limit exceeded)
- `500`: Internal Server Error
- `503`: Service Unavailable

### Error Response Format
```json
{
  "error": "error_code",
  "message": "Human readable error message",
  "details": {
    "field": "Additional error details"
  }
}
```

## Security Considerations

### Authentication
- All authenticated endpoints require valid JWT access tokens
- Tokens expire after 15 minutes and must be refreshed
- Refresh tokens are valid for 30 days
- Failed authentication attempts are rate limited

### Request Signing
- Turnkey API requests require P256 ECDSA signatures
- Signatures include request timestamp to prevent replay attacks
- Public keys are verified against registered credentials

### MEV Protection
- Transactions are routed through MEV protection services
- Multiple providers ensure redundancy and optimal routing
- Real-time monitoring of service health and performance

### Data Privacy
- All API communication uses TLS 1.3
- Sensitive data is encrypted at rest
- API logs exclude personally identifiable information
- Rate limiting prevents abuse and DoS attacks
