/** Stroops in 1 XLM (Number form — safe for display arithmetic). */
export const STROOPS_PER_XLM = 10_000_000;
/** Stroops in 1 XLM (BigInt form — required for precision-safe conversions). */
export const STROOPS_PER_XLM_BIGINT = BigInt(STROOPS_PER_XLM);
export const MIN_STROOPS = 1n;

/** Seconds in a day — mirrors MIN_INTERVAL_SECONDS in CONTRACT_LIMITS. */
export const SECONDS_PER_DAY = 86_400;
/** Seconds in a 7-day week. */
export const SECONDS_PER_WEEK = 604_800;
/** Seconds in a 30-day "month" (the billing cycle approximation used by the UI). */
export const SECONDS_PER_MONTH = 2_592_000;
// Generic UI ceiling: 9e15 stays within Number.MAX_SAFE_INTEGER for XLM conversion.
// Contract actions also enforce the lower caps below, mirrored from contract/src/lib.rs.
export const MAX_STROOPS = 9_000_000_000_000_000n;

export const BILLING_INTERVALS = [
  { label: "Daily", value: SECONDS_PER_DAY },
  { label: "Weekly", value: SECONDS_PER_WEEK },
  { label: "Monthly (~30d)", value: SECONDS_PER_MONTH },
] satisfies { label: string; value: number }[];

export const DEFAULT_EXPIRATION_LEDGER = 30;

export const CONTRACT_LIMITS = {
  MAX_PAY_PER_USE_AMOUNT: 100_000_000_000n,
  MAX_SUBSCRIPTION_AMOUNT: 100_000_000_000_000n,
  MIN_INTERVAL_SECONDS: SECONDS_PER_DAY,
  /** Max addresses per batch_pause_subscriptions call */
  MAX_BATCH_PAUSE: 25,
  /** Max addresses per whitelist_batch_add / whitelist_batch_remove call */
  MAX_BATCH_WHITELIST: 50,
} as const;

/** Hard cap on the number of entries persisted to the address book. */
export const MAX_ADDRESS_BOOK_ENTRIES = 250;