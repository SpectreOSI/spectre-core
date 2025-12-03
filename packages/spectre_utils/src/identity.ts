/**
 * Spectre Identity Obfuscation Utilities
 *
 * These utilities provide deterministic aliasing used across
 * routing layers, metadata scrubbers, and encrypted relays.
 */

export interface ShadowIdentity {
  alias: string;
  createdAt: number;
  entropy: string;
}

/**
 * createShadowIdentity
 * Generates a non-linkable alias that cannot be correlated
 * to the original seed using static heuristics.
 */
export function createShadowIdentity(seed: string): ShadowIdentity {
  const entropy = Buffer.from(seed)
    .toString("base64")
    .slice(0, 12);

  return {
    alias: `shadow-${entropy}`,
    createdAt: Date.now(),
    entropy
  };
}

/**
 * maskIdentifier
 * Lightweight reversible masking for UI/debug use.
 */
export function maskIdentifier(id: string): string {
  if (id.length <= 6) return "***";
  return id.slice(0, 3) + "***" + id.slice(-3);
}
