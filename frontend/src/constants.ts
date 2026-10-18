export const STROOPS_PER_XLM = 10_000_000;
export const MIN_STROOPS = 1n;
// Generic UI ceiling: 9e15 stays within Number.MAX_SAFE_INTEGER for XLM conversion.
// Contract actions also enforce the lower caps below, mirrored from contract/src/lib.rs.
export const MAX_STROOPS = 9_000_000_000_000_000n;

export const BILLING_INTERVALS = [
  { label: "Daily", value: 86_400 },
  { label: "Weekly", value: 604_800 },
  { label: "Monthly (~30d)", value: 2_592_000 },
] satisfies { label: string; value: number }[];

export const DEFAULT_EXPIRATION_LEDGER = 30;

/**
 * Hard limits enforced by the on-chain contract.
 *
 * Source of truth: `contract/src/caps.rs` (batch and page caps) together with
 * the amount/interval guards in `contract/src/lib.rs`. The operator-facing table
 * built from the batch caps lives in `docs/limits.md`; `node
 * scripts/emit-caps-table.mjs --check` fails the build's doc check when the two
 * disagree.
 *
 * Frontend callers must read these values from here rather than re-declaring
 * them, so a cap change on the contract side propagates to the UI in one place.
 */
export const CONTRACT_LIMITS = {
  /** Max `amount` accepted by `pay_per_use()` — contract/src/lib.rs. */
  MAX_PAY_PER_USE_AMOUNT: 100_000_000_000n,
  /** Max `amount` accepted by `subscribe()` — contract/src/lib.rs. */
  MAX_SUBSCRIPTION_AMOUNT: 100_000_000_000_000n,
  /** Min `interval` (seconds) accepted by `subscribe()` — one day. */
  MIN_INTERVAL_SECONDS: 86400,
  /**
   * Max addresses per `batch_pause_subscriptions` / `batch_cancel` call.
   * Mirrors `caps::MAX_BATCH_PAUSE_SUBSCRIPTIONS` in contract/src/caps.rs.
   */
  MAX_BATCH_PAUSE: 25,
  /**
   * Max addresses per `whitelist_batch_add` / `whitelist_batch_remove` call.
   * Mirrors `caps::MAX_WHITELIST_BATCH_SIZE` in contract/src/caps.rs.
   */
  MAX_BATCH_WHITELIST: 50,
} as const;

/** Hard cap on the number of entries persisted to the address book. */
export const MAX_ADDRESS_BOOK_ENTRIES = 250;