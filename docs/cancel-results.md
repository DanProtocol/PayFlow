# CancelResult Wire Encoding

## Overview

`CancelResult` is returned by `batch_cancel` to describe the outcome for each user address. It is a Soroban `#[contracttype]` enum encoded on the wire as a `ScVal::U32` discriminant.

## Wire Format

Each variant maps to a fixed `u32` discriminant:

| Discriminant | Variant | Meaning |
|--------------|---------|---------|
| `0` | `Cancelled` | Subscription was cancelled successfully (normal case) |
| `1` | `NoSubscription` | No subscription entry found for this address |
| `2` | `AlreadyCancelled` | Subscription was already inactive (`active = false`) |
| `3` | `PausedCancelled` | A paused (but active) subscription was cancelled |
| `4` | `MerchantFrozen` | Subscription belongs to a frozen merchant and was cancelled |

## Append-Only Invariant

**New variants MUST be appended at the end.** Off-chain consumers (keepers, indexers, alert scripts) that only know discriminants 0-2 will continue to decode those values correctly when newer contracts return 3 or 4.

Renaming or reordering existing discriminants is a **breaking change** for all off-chain parsers.

## Example XDR Encoding

A `Vec<CancelResult>` of `[Cancelled, PausedCancelled, MerchantFrozen]` encodes as:

```
ScVec [
  ScVal::U32(0),  // Cancelled
  ScVal::U32(3),  // PausedCancelled
  ScVal::U32(4),  // MerchantFrozen
]
```

## Handling in Off-Chain Code

### TypeScript/JavaScript Example

```typescript
enum CancelResult {
  Cancelled = 0,
  NoSubscription = 1,
  AlreadyCancelled = 2,
  PausedCancelled = 3,
  MerchantFrozen = 4,
}

function parseCancelResult(scval: xdr.ScVal): CancelResult | number {
  if (scval.switch() === xdr.ScValType.scvU32()) {
    const discriminant = scval.u32();
    
    // Handle known variants
    if (discriminant <= 4) {
      return discriminant as CancelResult;
    }
    
    // Unknown future variant - return raw discriminant
    console.warn(`Unknown CancelResult discriminant: ${discriminant}`);
    return discriminant;
  }
  
  throw new Error('Invalid CancelResult encoding');
}
```

### Rust Example

```rust
use soroban_sdk::xdr::{ScVal, ScValType};

fn decode_cancel_result(scval: &ScVal) -> Result<u32, String> {
    match scval {
        ScVal::U32(discriminant) => Ok(*discriminant),
        _ => Err("Expected U32 discriminant".to_string()),
    }
}

// In your keeper logic:
match decode_cancel_result(&result)? {
    0 => println!("Cancelled"),
    1 => println!("NoSubscription"),
    2 => println!("AlreadyCancelled"),
    3 => println!("PausedCancelled - investigate why subscription was paused"),
    4 => println!("MerchantFrozen - frozen merchant cancellation"),
    n => println!("Unknown discriminant {}", n),
}
```

## Backward Compatibility

### Before This Change

Old contracts returned only discriminants 0-2:
- `Cancelled (0)`
- `NoSubscription (1)`
- `AlreadyCancelled (2)`

A paused subscription or frozen merchant would report `Cancelled (0)` without distinction.

### After This Change

New contracts return discriminants 0-4:
- `Cancelled (0)` - only for routine cancellations
- `NoSubscription (1)` - unchanged
- `AlreadyCancelled (2)` - unchanged
- `PausedCancelled (3)` - **NEW** - was paused when cancelled
- `MerchantFrozen (4)` - **NEW** - merchant is frozen

**Compatibility guarantee:** Discriminants 0-2 have identical semantics. Off-chain parsers that pre-date this change will:
- ✅ Correctly decode `Cancelled`, `NoSubscription`, `AlreadyCancelled`
- ⚠️ Encounter unknown discriminants 3-4, which they should handle gracefully (log a warning, treat as "cancelled" generically, etc.)

## Usage in batch_cancel

The `batch_cancel` function checks subscription state before cancelling and returns the appropriate result:

1. **Priority 1:** Check if merchant is frozen → `MerchantFrozen`
2. **Priority 2:** Check if subscription is paused → `PausedCancelled`
3. **Default:** Regular cancellation → `Cancelled`

This priority order ensures that the most critical state (merchant frozen) is always reported, even if the subscription is also paused.

## Related Documentation

- `docs/charge-results.md` - Similar wire encoding for `ChargeResult`
- `scripts/EVENT-MAPPING.md` - Event and result type mappings
- `contract/src/batch.rs:20-27` - ChargeResult discriminant stability note

## Testing

See `contract/src/test.rs`:
- `test_batch_cancel_distinguishes_paused_cancellation`
- `test_batch_cancel_distinguishes_frozen_merchant`
- `test_batch_cancel_frozen_merchant_takes_precedence_over_paused`

## Summary

`CancelResult` uses an append-only discriminant scheme to maintain backward compatibility with off-chain parsers. New variants distinguish unusual cancellation states (paused subscriptions, frozen merchants) that admins need to review.
