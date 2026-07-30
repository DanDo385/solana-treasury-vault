# Portfolio Brief: Solana Treasury Vault

## Positioning

A CLI-first Anchor program that demonstrates controlled Devnet SOL treasury operations with separable authority, withdrawal proposals, cancellation, and a slot-based execution cooldown. The point is not to mint a token or show a counter. The point is to express a financial-control workflow as explicit, testable on-chain state transitions.

## Candidate portfolio card

- Title: Solana Treasury Vault
- Slug: `solana-treasury-vault`
- Status: complete, Devnet deployed
- Project type: CLI / non-interactive
- `demoUrl`: `null`
- Source: pending public GitHub repository creation
- Explorer: https://explorer.solana.com/address/G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri?cluster=devnet

## Suggested summary

Anchor/Rust treasury-control program on Solana Devnet. A treasury PDA holds SOL and control state; the owner proposes a fixed recipient and amount, a separate approver authorizes, and the owner executes only after the configured slot cooldown. Local-validator tests prove rejected self-approval, cancelled-proposal denial, and the valid execution path. The published Devnet binary and on-chain IDL are independently matched to local build artifacts.

## Evidence

- Program ID: `G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri`
- ProgramData: `CruNnB7ysxfaU3GvmiU6twzcYFi58nhWoRKnypRa9Q49`
- IDL metadata PDA: `GmxzBq65RvwJQDMKesqweEFjPnMpojS8u5yVwpzcSiaL`
- On-chain/local binary SHA-256: `d90d22f6e48db11ccc64990f29d55d7c96bf554317fa43ab96e9a2c9f4d24810`
- Local verification: `cargo fmt --check` and `anchor test` pass

## Case-study structure

### Problem

A treasury needs a separation between proposal and approval. A single signer should not be able to define and authorize a withdrawal, and cancellation must remain terminal.

### Design

- `Treasury` PDA stores owner, approver, cooldown slots, proposal nonce, and bump.
- `WithdrawalProposal` PDA fixes treasury, recipient, amount, proposal slot, lifecycle flags, and bump.
- Program state retains rent-exempt lamports before a withdrawal can execute.

### Trade-offs

- V1 transfers native SOL only. SPL tokens and associated token accounts are intentionally deferred.
- Cooldown is slot-based, which is deterministic on-chain but less human-readable than a timestamp-based policy.
- The upgrade authority remains active on the dedicated Devnet deployer. This is appropriate for an evolving portfolio project, not an immutability claim.

### Limitations

- Devnet only. No mainnet, audit, custody, or production-ready claim.
- No governance, multisig, SPL-token, emergency-pauser, or time-lock administration surface.
- Current demonstration is CLI-first. A visual dashboard should be built only after it adds decision-quality evidence rather than decoration.

## Media plan

1. Capture terminal frames for build, local test, Devnet program state, and IDL/hash verification.
2. Generate a short GIF from descriptive screenshots under `public/screenshots/`.
3. Record a 2 to 3 minute terminal walkthrough, upload unlisted, and add the returned URL to `public/media.json`.
4. Add the portfolio card only after source is public on GitHub and the project-owned media kit exists on the remote `main` ref.
