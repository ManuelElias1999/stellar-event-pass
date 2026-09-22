# Stellar Event Pass

Contrato inteligente en Soroban para **Stellar Elite Bolivia**. El ledger registra
qué address compró cada pase numerado y garantiza que solo su propietario pueda
utilizarlo, una única vez.


## Despliegue verificado en Testnet

- **Contract ID:** `CDWTEUQI3WULFFXAQ22A2VFT22GGA3OEKT5L7XL62TJHKYHHMGQIXONH`
- **Buyer:** `GDF7XZHJ3QGEKGSZUVILGR64FLI3FZHO5CMPHXSL3EBJP6UAY6S4PGIG`
- **Pass ID:** `101`
- **Antes del canje:** `is_valid = true`
- **Después del canje:** `is_valid = false`
- **Estado final:** `redeemed = true`, compra en el ledger `4802546` y canje en el ledger `4802547`.
- [Ver contrato en Stellar Expert](https://stellar.expert/explorer/testnet/contract/CDWTEUQI3WULFFXAQ22A2VFT22GGA3OEKT5L7XL62TJHKYHHMGQIXONH)
- [Ver ejecución exitosa en GitHub Actions](https://github.com/ManuelElias1999/stellar-event-pass/actions/runs/35671247634)

## Funciones

- `purchase(buyer, pass_id)`: compra un pase libre y exige la firma de `buyer`.
- `get_pass(pass_id)`: devuelve propietario, ledger de compra y estado del pase.
- `is_valid(pass_id)`: indica si el pase existe y todavía puede utilizarse.
- `redeem(holder, pass_id)`: exige la firma del propietario y marca el pase como usado.
- Eventos `PassPurchased` y `PassRedeemed`, visibles en el Explorer.

## Garantías del contrato

1. Un número de pase solo puede venderse una vez.
2. Solo la address registrada como `holder` puede canjearlo.
3. Un pase canjeado no puede volver a utilizarse.
4. La compra y el canje dejan eventos y estado persistente en el ledger.

## Requisitos

- Rust 1.84 o superior.
- Target `wasm32v1-none`.
- Stellar CLI.

```bash
rustup target add wasm32v1-none
cargo install --locked stellar-cli
```

## Pruebas y compilación

```bash
cargo test --workspace
stellar contract build --package event-pass
```

El WASM se genera en:

```text
target/wasm32v1-none/release/event_pass.wasm
```

## Despliegue en Testnet

```bash
stellar keys generate elite-demo --network testnet --fund

CONTRACT_ID=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/event_pass.wasm \
  --source-account elite-demo \
  --network testnet)

echo "$CONTRACT_ID"
```

## Demostración manual

```bash
BUYER=$(stellar keys address elite-demo)

# 1. Comprar el pase 101
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account elite-demo \
  --network testnet \
  -- purchase \
  --buyer "$BUYER" \
  --pass_id 101

# 2. Verificar que está activo: devuelve true
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account elite-demo \
  --network testnet \
  -- is_valid \
  --pass_id 101

# 3. Canjear el pase
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account elite-demo \
  --network testnet \
  -- redeem \
  --holder "$BUYER" \
  --pass_id 101

# 4. Ver el estado final: redeemed será true
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source-account elite-demo \
  --network testnet \
  -- get_pass \
  --pass_id 101
```

También se puede ejecutar todo el flujo con:

```bash
chmod +x scripts/demo.sh
./scripts/demo.sh "$CONTRACT_ID" elite-demo 101
```

## Explorer

Después del despliegue, abre:

```text
https://stellar.expert/explorer/testnet/contract/CDWTEUQI3WULFFXAQ22A2VFT22GGA3OEKT5L7XL62TJHKYHHMGQIXONH
```

En **Contract Activity / Events** se verán `PassPurchased` y `PassRedeemed`.

## Entregable

El guion cronometrado está en [`docs/GUION_VIDEO.md`](docs/GUION_VIDEO.md).
