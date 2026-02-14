#!/bin/bash
# Bitcoin DLD REST API Test Script
# This script demonstrates all available API endpoints

API_URL="http://localhost:3000"

echo "Bitcoin Digital Labor Derivative REST API Demo"
echo "=============================================="
echo ""

# Test health endpoint
echo "1. Health Check"
echo "   GET ${API_URL}/health"
curl -s "${API_URL}/health" | jq .
echo ""

# Test RBI endpoint
echo "2. Current RBI Status"
echo "   GET ${API_URL}/api/v1/rbi"
curl -s "${API_URL}/api/v1/rbi" | jq .
echo ""

# Test pool balance endpoint
echo "3. Pool Balance"
echo "   GET ${API_URL}/api/v1/pool/balance"
curl -s "${API_URL}/api/v1/pool/balance" | jq .
echo ""

# Test dividend calculation with default parameters
echo "4. Calculate Dividend (default parameters)"
echo "   GET ${API_URL}/api/v1/participants/alice/dividend"
curl -s "${API_URL}/api/v1/participants/alice/dividend" | jq .
echo ""

# Test dividend calculation with custom parameters
echo "5. Calculate Dividend (custom parameters)"
echo "   GET ${API_URL}/api/v1/participants/bob/dividend?stake_amount_sats=500000000&trust_coefficient=2.0&velocity_multiplier=1.5"
curl -s "${API_URL}/api/v1/participants/bob/dividend?stake_amount_sats=500000000&trust_coefficient=2.0&velocity_multiplier=1.5" | jq .
echo ""

# Test velocity endpoint
echo "6. Participant Velocity Data"
echo "   GET ${API_URL}/api/v1/participants/charlie/velocity"
curl -s "${API_URL}/api/v1/participants/charlie/velocity" | jq .
echo ""

echo "Demo complete!"
echo ""
echo "To start the API server, run:"
echo "  cargo run --bin api-server --features api"
