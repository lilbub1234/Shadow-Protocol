#!/bin/bash
# Shade Framework - Ethereum Deployment Script
# Copyright (c) 2025 Shadow Protocol Contributors

set -e

echo "========================================="
echo "Shade Framework - Ethereum Deployment"
echo "========================================="

# Configuration
NETWORK=${NETWORK:-"goerli"}
RPC_URL=${RPC_URL:-"https://eth-goerli.g.alchemy.com/v2/YOUR_KEY"}
PRIVATE_KEY=${PRIVATE_KEY:-""}

if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY environment variable not set"
    exit 1
fi

echo "Network: $NETWORK"
echo "RPC URL: $RPC_URL"
echo ""

# Check if forge is installed
if ! command -v forge &> /dev/null; then
    echo "Error: Foundry not installed. Install from https://getfoundry.sh"
    exit 1
fi

# Navigate to contracts directory
cd "$(dirname "$0")/../../verifiers/solidity"

echo "Compiling contracts..."
forge build

echo ""
echo "Deploying Groth16 Verifier..."
VERIFIER_ADDRESS=$(forge create \
    --rpc-url "$RPC_URL" \
    --private-key "$PRIVATE_KEY" \
    src/Groth16Verifier.sol:Groth16Verifier \
    --json | jq -r '.deployedTo')

if [ -z "$VERIFIER_ADDRESS" ]; then
    echo "Error: Failed to deploy verifier"
    exit 1
fi

echo "✓ Verifier deployed at: $VERIFIER_ADDRESS"

echo ""
echo "Deploying Privacy Application..."
APPLICATION_ADDRESS=$(forge create \
    --rpc-url "$RPC_URL" \
    --private-key "$PRIVATE_KEY" \
    --constructor-args "$VERIFIER_ADDRESS" \
    src/Groth16Verifier.sol:PrivacyApplication \
    --json | jq -r '.deployedTo')

if [ -z "$APPLICATION_ADDRESS" ]; then
    echo "Error: Failed to deploy application"
    exit 1
fi

echo "✓ Application deployed at: $APPLICATION_ADDRESS"

# Save deployment info
DEPLOYMENT_FILE="deployments/${NETWORK}.json"
mkdir -p deployments
cat > "$DEPLOYMENT_FILE" <<EOF
{
  "network": "$NETWORK",
  "chainId": $(cast chain-id --rpc-url "$RPC_URL"),
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "contracts": {
    "Groth16Verifier": {
      "address": "$VERIFIER_ADDRESS",
      "blockNumber": $(cast block-number --rpc-url "$RPC_URL")
    },
    "PrivacyApplication": {
      "address": "$APPLICATION_ADDRESS",
      "blockNumber": $(cast block-number --rpc-url "$RPC_URL")
    }
  }
}
EOF

echo ""
echo "========================================="
echo "Deployment Complete!"
echo "========================================="
echo "Verifier:     $VERIFIER_ADDRESS"
echo "Application:  $APPLICATION_ADDRESS"
echo "Details saved to: $DEPLOYMENT_FILE"
echo ""
echo "Verify contracts on Etherscan:"
echo "forge verify-contract --chain-id $(cast chain-id --rpc-url "$RPC_URL") $VERIFIER_ADDRESS Groth16Verifier"
echo "========================================="
