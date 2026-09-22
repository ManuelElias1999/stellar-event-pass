#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  echo "Uso: ./scripts/demo.sh <CONTRACT_ID> <IDENTITY> [PASS_ID]"
  exit 1
fi

CONTRACT_ID="$1"
IDENTITY="$2"
PASS_ID="${3:-101}"
BUYER="$(stellar keys address "$IDENTITY")"

echo "1/4 Comprando el pase $PASS_ID para $BUYER"
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account "$IDENTITY" \
  --network testnet \
  -- purchase \
  --buyer "$BUYER" \
  --pass_id "$PASS_ID"

echo "2/4 Comprobando que el pase está activo"
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --network testnet \
  -- is_valid \
  --pass_id "$PASS_ID"

echo "3/4 Canjeando el pase"
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account "$IDENTITY" \
  --network testnet \
  -- redeem \
  --holder "$BUYER" \
  --pass_id "$PASS_ID"

echo "4/4 Consultando el estado final"
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --network testnet \
  -- get_pass \
  --pass_id "$PASS_ID"

echo "Explorer del contrato:"
echo "https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID"
