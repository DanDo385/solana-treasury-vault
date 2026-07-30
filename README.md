# Solana Treasury Vault

A Devnet-only Anchor program for controlled native-SOL treasury operations. The program separates the person who proposes and executes a withdrawal from the person who approves it, then records every request as explicit on-chain state.

**Status:** Devnet deployment verified. Local lifecycle tests pass. A general-purpose command-line client is the next product milestone.

- **Program ID:** `G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri`
- **Devnet Explorer:** https://explorer.solana.com/address/G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri?cluster=devnet
- **Deployment evidence:** [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)

## What it does

A Treasury account is a program-derived address, or PDA. It stores an owner, an independent approver, a slot-based cooldown, and a monotonically increasing proposal number.

A withdrawal follows this sequence:

1. The owner initializes a treasury with a different approver.
2. The owner funds the treasury with native SOL.
3. The owner proposes one recipient and one amount.
4. The designated approver approves that specific proposal.
5. After the cooldown, the owner executes the approved proposal.
6. The owner may cancel a proposal before execution.

The program rejects self-approval, approval by any other signer, execution before approval, execution before cooldown maturity, and every action against a cancelled proposal. It preserves the treasury account's rent-exempt reserve.

## Roles

| Role | Authority |
| --- | --- |
| Owner | Initialize, fund, propose, execute mature approved withdrawals, cancel proposals. |
| Approver | Approve a pending proposal. Must differ from the owner. |
| Recipient | Receives SOL only when the owner executes an approved, mature proposal. |
| Program | Enforces the state machine and PDA constraints. |

## Quick start

### Prerequisites

This repository is tested with:

- Solana CLI 4.1.1
- Anchor CLI 1.1.2
- Rust 1.89.0

Run from the project root:

```bash
cd ~/Code/solana-treasury-vault
```

The project includes a safe shell launcher. It avoids a machine-local compiler wrapper that can interfere with Cargo builds.

```bash
./scripts/cmux-safe-shell verify
```

That command runs the complete local-validator lifecycle test. It builds the program, starts a local validator, deploys the program locally, tests the treasury controls, then tears the validator down.

For an interactive development shell:

```bash
./scripts/cmux-safe-shell shell
anchor build
anchor test
```

## What the automated test proves

The integration test creates real local accounts and proves:

- treasury initialization requires distinct owner and approver keys
- owner self-approval fails
- the independent approver can approve a proposal
- the owner can execute an approved withdrawal
- the owner can cancel a proposal
- approval after cancellation fails
- execution after cancellation fails

The test lives at [tests/src/test_initialize.rs](tests/src/test_initialize.rs).

## Inspect the Devnet program

The deployed program is public on Devnet. This checks the current on-chain program record without sending a transaction:

```bash
solana -u devnet program show G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri
```

The Devnet deployment is for demonstration and learning. It holds no production funds.

## Current user interface

The current user interface is the tested Anchor integration flow. A reusable `treasuryctl` command-line client does not exist yet.

That distinction matters: the program is deployed and verified, while direct day-to-day treasury operation still requires the next development phase. Do not treat the Devnet deployment as a production treasury service.

## Development roadmap

1. Publish the verified source and add continuous integration.
2. Build `treasuryctl` with `initialize`, `fund`, `propose`, `approve`, `execute`, `cancel`, and `status` commands.
3. Add end-to-end Devnet tests driven by that client, including a real cooldown test.
4. Capture live CLI and Devnet Explorer evidence for the portfolio media kit.
5. Add the repository-backed portfolio card after its source media is published.

## Safety boundaries

- Localnet is the development and automated-test environment.
- Devnet is a public demonstration environment only.
- Mainnet-beta is out of scope.
- Never commit a wallet keypair, mnemonic, `.env` file, or 1Password reference.
- Do not put material funds into this program. It is a learning and portfolio project, not an audited custody system.

## License

No license has been selected yet.
