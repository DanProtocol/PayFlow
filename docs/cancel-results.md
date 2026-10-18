# CancelResult Wire Encoding

Reference for the `CancelResult` outcome values that `batch_cancel` returns.
Similar to `ChargeResult` (documented in [`charge-results.md`](charge-results.md)),
but simpler: no aggregate event, and fewer variants.

**Scope.** Byte-level encoding and variant semantics. For batch cancellation
workflow see [`API.md`](API.md) (`batch_cancel`).

**Related docs:**
[`charge-results.md`](charge-results.md) (ChargeResult encoding) |
[`ERROR-CODES.md`](ERROR-CODES.md) (contract errors) |
[`API.md`](API.md) (API reference)

---

## Table of Contents

- [Cheat sheet](#cheat-sheet)
- [The return value](#the-return-value)
  - [The variant table](#the-variant-table)
  - [XDR encoding](#xdr-encoding)
- [Why these variants matter](#why-these-variants-matter)
- [Append-only rule](#append-only-rule)

---

## Cheat sheet

| Question                                  | Answer                                                                                     |
| ----------------------------------------- | ------------------------------------------------------------------------------------------ |
| What XDR type is one `CancelResult`?       | `scvU32` carrying the variant discriminant. **Not** `scvSymbol`.                              |
| What is the whole return value?            | `scvVec` of those `scvU32`, one element per input address, in input order.                   |
| How many variants?                         | 5, discriminants `0`..`4`, append-only.                                                      |
| Does `batch_cancel` emit an aggregate event? | No. Unlike `batch_charge`, there is no summary event.                                      |
| What does an aborted transaction return?   | `scvVoid`. No `CancelResult` exists at all.                                                  |

---

## The return value

`batch_cancel(users) -> Vec<CancelResult>` returns one `CancelResult` per input
address, **in the same order as the input**. The result for address `i` in the
returned vector always corresponds to `users[i]`.

`CancelResult` is a Soroban `#[contracttype]` enum with no payload fields (see
[`contract/src/batch.rs`](../contract/src/batch.rs)). A fieldless
`#[contracttype]` enum serializes as a `u32` holding the variant index. So each
element is an `scvU32`.

### The variant table

The discriminants are the declaration order in `CancelResult`. They are
append-only: adding a variant means appending, and renumbering an existing one
breaks every off-chain decoder.

| Discriminant | Rust variant             | Meaning                                                                                             | Was cancelled? |
| -----------: | ------------------------ | --------------------------------------------------------------------------------------------------- | -------------- |
| `0`          | `Cancelled`              | Subscription was cancelled successfully.                                                            | Yes            |
| `1`          | `NoSubscription`         | No subscription found for this address.                                                            | No             |
| `2`          | `AlreadyCancelled`       | Subscription was already cancelled.                                                                | No (idempotent)|
| `3`          | `PausedCancelled`        | Subscription was paused; cancel succeeded but state was unusual.                                    | Yes            |
| `4`          | `MerchantFrozen`         | Subscription's merchant was frozen; cancel succeeded but state was unusual.                         | Yes            |

**New in this version:** `PausedCancelled` and `MerchantFrozen` (discriminants 3–4)
allow an admin operating a batch cancel to distinguish routine cancels from those
that hit a paused subscription or a frozen-merchant subscription, which are
exactly the cases worth reviewing.

All variants except `NoSubscription` emit a `cancelled` event. Variants 0, 3, and 4
all successfully cancel the subscription; the distinction is for auditing purposes.

### XDR encoding

The encoding is identical to `ChargeResult`: a `scvVec` of `scvU32` discriminants.

Example: three addresses submitted, producing `Cancelled`, `PausedCancelled`, and
`MerchantFrozen`:

```
scvVec[ scvU32(0), scvU32(3), scvU32(4) ]
```

base64:

```
AAAAEAAAAAEAAAADAAAAAwAAAAAAAAADAAAAAQAAAAMAAAAE
```

See [`charge-results.md`](charge-results.md) for the full XDR breakdown. The
layout is the same; only the discriminant meanings differ.

Single elements:

| Value                              | base64         | hex                              |
| ---------------------------------- | -------------- | -------------------------------- |
| `scvU32(0)` (`Cancelled`)          | `AAAAAwAAAAA=` | `00000003 00000000`              |
| `scvU32(1)` (`NoSubscription`)     | `AAAAAwAAAAE=` | `00000003 00000001`              |
| `scvU32(2)` (`AlreadyCancelled`)   | `AAAAAwAAAAI=` | `00000003 00000002`              |
| `scvU32(3)` (`PausedCancelled`)    | `AAAAAwAAAAM=` | `00000003 00000003`              |
| `scvU32(4)` (`MerchantFrozen`)     | `AAAAAwAAAAQ=` | `00000003 00000004`              |
| `scvVoid` (no return value)        | `AAAAAQ==`     | `00000001`                       |

---

## Why these variants matter

Before variants 3–4 were added, a paused subscription and a merchant-frozen
subscription both reported `Cancelled` (discriminant 0), with no signal that the
state was unusual.

**Problem:** An admin operating a batch cancel cannot distinguish a routine cancel
from one that hit a paused or frozen subscription, which is exactly the case worth
reviewing.

**Solution:** The new variants allow off-chain tooling to flag these for review
without requiring a separate state query for every cancelled address.

**Precedence:** When both conditions are true (paused *and* merchant frozen),
`MerchantFrozen` (4) takes precedence over `PausedCancelled` (3), because the
merchant freeze is the more critical operational state to surface.

---

## Append-only rule

When adding new variants, **always append** at the end so existing off-chain
parsers continue to recognize the original discriminants 0–4. Renaming or
reordering is a breaking change for all consumers.

This rule is documented in the enum's doc comment in `contract/src/batch.rs` and
mirrors the `ChargeResult` append-only convention.

---

## Decoding example (TypeScript)

```ts
import { xdr } from "@stellar/stellar-sdk";

/** Discriminant order in `contract/src/batch.rs`. Append-only. */
const CANCEL_RESULTS = [
  "Cancelled",
  "NoSubscription",
  "AlreadyCancelled",
  "PausedCancelled",
  "MerchantFrozen",
] as const;

export type CancelResultName = (typeof CANCEL_RESULTS)[number];

export function decodeCancelResults(
  retval: xdr.ScVal | undefined | null,
): CancelResultName[] | null {
  if (!retval) return null;
  if (retval.switch().name !== "scvVec") return null;

  return retval.vec().map((item) => {
    const type = item.switch().name;
    if (type !== "scvU32") {
      throw new Error(`expected scvU32 CancelResult, got ${type}`);
    }
    const n = item.u32();
    const name = CANCEL_RESULTS[n];
    if (!name) {
      throw new Error(`unknown CancelResult discriminant ${n}`);
    }
    return name;
  });
}
```

---

## Verifying this document

From the repository root:

```bash
# The variant list and its declaration order
sed -n '/pub enum CancelResult/,/^}/p' contract/src/batch.rs

# The batch_cancel function that produces these
rg -A 40 "pub fn batch_cancel" contract/src/batch.rs
```
