# Guion del video — máximo 3 minutos

Duración objetivo: **2:45–2:55**.

## 0:00–0:20 — Introducción

> Hola, soy Manuel Elias y este es mi entregable para Stellar Elite Bolivia. Creé
> un Event Pass en Soroban que registra qué address compró cada pase y garantiza
> que solamente su propietario pueda utilizarlo, una única vez.

## 0:20–0:45 — Contrato

Mostrar brevemente `contracts/event-pass/src/lib.rs`.

> El contrato tiene cuatro funciones. `purchase` registra al comprador y exige su
> autorización; `get_pass` consulta el estado; `is_valid` verifica si el pase sigue
> activo; y `redeem` comprueba que quien firma sea el propietario y que el pase no
> haya sido usado. La compra y el canje también publican eventos.

## 0:45–1:00 — Pruebas

Ejecutar:

```bash
cargo test --workspace
```

> Las cuatro pruebas pasan: flujo completo, compra duplicada, intento de uso por
> otra address y doble canje.

## 1:00–1:30 — Compra

Ejecutar el comando `purchase` preparado en el historial de la terminal.

> Ahora invoco `purchase` para el pase 101 desde mi cuenta de Testnet. La
> transacción fue exitosa y el estado devuelve mi address como holder, junto al
> ledger de compra y `redeemed` en falso.

## 1:30–1:45 — Validación

Ejecutar `is_valid`.

> Consulto `is_valid` y devuelve `true`, porque el pase existe y todavía no fue
> utilizado.

## 1:45–2:15 — Canje y estado final

Ejecutar `redeem` y después `get_pass`.

> Canjeo el pase con la misma address. El contrato verifica la firma y cambia el
> estado. Al consultar nuevamente, `redeemed` aparece en `true` y también queda
> registrado el ledger del canje. Si intentara repetirlo, el contrato devolvería
> `AlreadyRedeemed`.

## 2:15–2:35 — Explorer

Abrir Stellar Expert en la actividad del contrato.

> En el Explorer se ven las invocaciones y los eventos de compra y canje, por lo
> que el resultado no depende de una base de datos externa: quedó registrado en
> el ledger de Stellar.

## 2:35–2:55 — Reflexión

> Lo siguiente que necesito seguir aprendiendo es cómo integrar activos de
> Stellar para cobrar el pase, cómo administrar la expiración del almacenamiento
> con TTL y cómo conectar el contrato a una interfaz web que firme las
> transacciones con una wallet. Esto permitiría convertir el prototipo en una
> aplicación completa para eventos reales.

## Preparación antes de grabar

1. Aumentar el tamaño de la terminal y ocultar notificaciones.
2. Tener abiertos VS Code, la terminal y el Explorer.
3. Preparar los comandos en el historial para no escribir durante el video.
4. Usar un `pass_id` nuevo en cada toma.
5. Grabar una prueba corta y confirmar que texto y audio sean legibles.
