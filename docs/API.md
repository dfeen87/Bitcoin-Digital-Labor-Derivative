# Bitcoin DLD REST API

A REST API server that provides global node access to the Bitcoin Digital Labor Derivative protocol components.

## Features

- **Global Node Access**: Centralized access to all core protocol components (RBI Engine, Participant Registry, Distribution Pool)
- **RESTful Endpoints**: Clean HTTP API for querying protocol state
- **Real-time RBI Monitoring**: Get current Recession Bypass Index status
- **Dividend Calculations**: Calculate dividends for participants
- **Pool Balance Tracking**: Monitor the current distribution pool balance

## Quick Start

### Build and Run

```bash
# Build the API server
cargo build --bin api-server --features api --release

# Run the server
cargo run --bin api-server --features api
```

The server will start on `http://0.0.0.0:3000` by default.

## API Endpoints

### Health Check

**GET** `/health`

Returns the health status of the API server.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### Get RBI Status

**GET** `/api/v1/rbi`

Returns the current Recession Bypass Index calculation.

**Response:**
```json
{
  "rbi_value": 1.23,
  "status": "Healthy",
  "is_healthy": true,
  "timestamp": "2026-02-14T01:58:32.946797876+00:00",
  "components": {
    "v_dld": 1.15,
    "t_c": 0.92,
    "d_s": 0.08,
    "productivity_a": 1.42
  }
}
```

**Status Values:**
- `Healthy`: RBI is within normal range
- `Warning`: RBI is approaching critical levels
- `Critical`: RBI indicates economic stress
- `Indeterminate`: Insufficient data for calculation
- `Invalid`: Invalid state or parameters

### Get Pool Balance

**GET** `/api/v1/pool/balance`

Returns the current distribution pool balance.

**Response:**
```json
{
  "balance_sats": 1000000000,
  "balance_btc": 10.0
}
```

### Calculate Participant Dividend

**GET** `/api/v1/participants/:id/dividend`

Calculate the dividend for a specific participant.

**Query Parameters:**
- `stake_amount_sats` (optional): Stake amount in satoshis (default: 100000000)
- `trust_coefficient` (optional): Trust coefficient multiplier (default: 1.0)
- `velocity_multiplier` (optional): Velocity multiplier (default: 1.0)

**Example:**
```bash
curl "http://localhost:3000/api/v1/participants/alice/dividend?stake_amount_sats=100000000&trust_coefficient=1.5&velocity_multiplier=1.2"
```

**Response:**
```json
{
  "participant_id": "alice",
  "dividend_sats": 1200000000,
  "dividend_btc": 12.0,
  "stake_amount_sats": 100000000,
  "trust_coefficient": 1.5,
  "velocity_multiplier": 1.2
}
```

### Get Participant Velocity

**GET** `/api/v1/participants/:id/velocity`

Get velocity data for a specific participant.

**Response:**
```json
{
  "participant_id": "bob",
  "velocity_multiplier": 1.0,
  "average_utxo_age_days": 30.0,
  "transaction_count": 5
}
```

## Architecture

The API server is built with:

- **Axum**: Modern web framework for Rust
- **Tokio**: Async runtime
- **Tower**: Middleware framework
- **Serde**: Serialization/deserialization

### Global Node

The `GlobalNode` structure provides centralized access to:

- **RBI Engine**: Calculates the Recession Bypass Index
- **Participant Registry**: SQLite-backed participant data
- **Pool Balance**: Current distribution pool balance

The global node is thread-safe and shared across all API handlers using Arc and Mutex.

## Development

### Testing the API

```bash
# Health check
curl http://localhost:3000/health

# Get RBI status
curl http://localhost:3000/api/v1/rbi

# Get pool balance
curl http://localhost:3000/api/v1/pool/balance

# Calculate dividend
curl "http://localhost:3000/api/v1/participants/alice/dividend?stake_amount_sats=100000000"

# Get velocity data
curl http://localhost:3000/api/v1/participants/bob/velocity
```

### Building

The API feature must be enabled when building:

```bash
cargo build --features api
```

### Running Tests

```bash
cargo test --features api
```

## Configuration

Currently, the server uses default configuration:
- Host: `0.0.0.0`
- Port: `3000`
- Pool Balance: 1,000,000,000 sats (10 BTC)

Future versions will support configuration via environment variables or config files.

## Security Considerations

- **Read-Only**: Current implementation is read-only (no mutations)
- **CORS**: Permissive CORS is enabled for development (should be configured for production)
- **No Authentication**: Authentication/authorization to be added in future versions
- **Rate Limiting**: Not currently implemented

## Future Enhancements

- [ ] Authentication and authorization
- [ ] Rate limiting
- [ ] WebSocket support for real-time updates
- [ ] Prometheus metrics endpoint
- [ ] Configuration file support
- [ ] TLS/HTTPS support
- [ ] Database persistence for pool state
- [ ] Integration with Bitcoin Core RPC
- [ ] Participant registration endpoints
- [ ] Historical data queries

## License

MIT License - see [LICENSE](../LICENSE)
