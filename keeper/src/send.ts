/**
 * TODO(T18, T19): send via NOWNodes. Poll getSignatureStatuses every 1.5s.
 * `--skip-preflight` is required for the STRCx MintNotApproved proof so the failure lands on-chain.
 */
export function sendTransaction(_serialized: Uint8Array, _skipPreflight: boolean): Promise<string> {
  throw new Error("TODO(T18): sendTransaction");
}
