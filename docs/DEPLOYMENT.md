# Devnet Deployment Evidence

## Published program

| Field | Value |
| --- | --- |
| Cluster | Solana Devnet |
| Program ID | `G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri` |
| ProgramData account | `CruNnB7ysxfaU3GvmiU6twzcYFi58nhWoRKnypRa9Q49` |
| IDL metadata PDA | `GmxzBq65RvwJQDMKesqweEFjPnMpojS8u5yVwpzcSiaL` |
| Upgrade authority | `3YKNJihduxaYC9AvZDLZE8GCAi8jjaPPiKwTMyiEnvSK` |
| Initial deployment slot | `479829317` |
| Program size | `208744` bytes |
| Compiled/on-chain SHA-256 | `d90d22f6e48db11ccc64990f29d55d7c96bf554317fa43ab96e9a2c9f4d24810` |

Explorer: https://explorer.solana.com/address/G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri?cluster=devnet

## Verified properties

- The Devnet program is owned by Solana's upgradeable BPF loader.
- Its upgrade authority is the dedicated Devnet-only deployer, not the local test payer.
- A binary dump from Devnet has the same SHA-256 as `target/deploy/solana_treasury_vault.so`.
- The canonical Anchor IDL fetched from Devnet equals the generated IDL after normalized JSON comparison.
- `Anchor.toml` maps the same Program ID under both `localnet` and `devnet`, while Localnet remains the default provider for automated tests.

## Reproduce verification

```bash
cd ~/Code/solana-treasury-vault
anchor test
solana -u devnet program show G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri
solana -u devnet program dump \
  G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri \
  /tmp/solana_treasury_vault-devnet.so
shasum -a 256 target/deploy/solana_treasury_vault.so /tmp/solana_treasury_vault-devnet.so
```

Never put deployer keypair JSON in this repository, terminal transcripts, screenshots, or public documentation.
