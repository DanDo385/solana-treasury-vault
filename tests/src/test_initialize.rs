use anchor_client::{Client, Cluster, CommitmentConfig};
use solana_keypair::{read_keypair_file, Keypair};
use solana_pubkey::Pubkey;
use solana_sdk_ids::system_program;
use solana_signer::Signer;

const FUNDING_LAMPORTS: u64 = 1_000_000;
const WITHDRAWAL_LAMPORTS: u64 = 500_000;

fn proposal_address(program_id: &Pubkey, treasury: &Pubkey, id: u64) -> Pubkey {
    Pubkey::find_program_address(
        &[
            solana_treasury_vault::constants::PROPOSAL_SEED,
            treasury.as_ref(),
            &id.to_le_bytes(),
        ],
        program_id,
    )
    .0
}

#[test]
fn treasury_requires_independent_approval_before_execution() {
    let program_id = Pubkey::try_from("G8Xqa1mKCLdQxYnqTMZgCUAB1hiiGMiNuVrtYHJQ5Xri").unwrap();
    let wallet_path = std::env::var("ANCHOR_WALLET").expect("Anchor must provide a local wallet");
    let owner = read_keypair_file(&wallet_path).expect("local test wallet must be readable");
    let approver = Keypair::new();
    let client = Client::new_with_options(Cluster::Localnet, &owner, CommitmentConfig::confirmed());
    let program = client.program(program_id).unwrap();
    let treasury = Pubkey::find_program_address(
        &[solana_treasury_vault::constants::TREASURY_SEED],
        &program_id,
    )
    .0;
    let proposal = proposal_address(&program_id, &treasury, 0);

    program
        .request()
        .accounts(solana_treasury_vault::accounts::Initialize {
            owner: owner.pubkey(),
            treasury,
            system_program: system_program::id(),
        })
        .args(solana_treasury_vault::instruction::Initialize {
            approver: approver.pubkey(),
            cooldown_slots: 0,
        })
        .send()
        .expect("owner can initialize a treasury with a separate approver");

    program
        .request()
        .accounts(solana_treasury_vault::accounts::FundTreasury {
            depositor: owner.pubkey(),
            treasury,
            system_program: system_program::id(),
        })
        .args(solana_treasury_vault::instruction::FundTreasury {
            amount: FUNDING_LAMPORTS,
        })
        .send()
        .expect("owner can fund the treasury");

    program
        .request()
        .accounts(solana_treasury_vault::accounts::ProposeWithdrawal {
            owner: owner.pubkey(),
            treasury,
            proposal,
            system_program: system_program::id(),
        })
        .args(solana_treasury_vault::instruction::ProposeWithdrawal {
            recipient: owner.pubkey(),
            amount: WITHDRAWAL_LAMPORTS,
        })
        .send()
        .expect("owner can create a withdrawal proposal");

    let self_approval = program
        .request()
        .accounts(solana_treasury_vault::accounts::ApproveWithdrawal {
            approver: owner.pubkey(),
            treasury,
            proposal,
        })
        .args(solana_treasury_vault::instruction::ApproveWithdrawal {})
        .send();
    assert!(
        self_approval.is_err(),
        "owner must not be able to self-approve"
    );

    program
        .request()
        .signer(&approver)
        .accounts(solana_treasury_vault::accounts::ApproveWithdrawal {
            approver: approver.pubkey(),
            treasury,
            proposal,
        })
        .args(solana_treasury_vault::instruction::ApproveWithdrawal {})
        .send()
        .expect("independent approver can approve the proposal");

    program
        .request()
        .accounts(solana_treasury_vault::accounts::ExecuteWithdrawal {
            owner: owner.pubkey(),
            treasury,
            proposal,
            recipient: owner.pubkey(),
        })
        .args(solana_treasury_vault::instruction::ExecuteWithdrawal {})
        .send()
        .expect("owner can execute a mature approved proposal");

    let cancelled_proposal = proposal_address(&program_id, &treasury, 1);
    program
        .request()
        .accounts(solana_treasury_vault::accounts::ProposeWithdrawal {
            owner: owner.pubkey(),
            treasury,
            proposal: cancelled_proposal,
            system_program: system_program::id(),
        })
        .args(solana_treasury_vault::instruction::ProposeWithdrawal {
            recipient: owner.pubkey(),
            amount: WITHDRAWAL_LAMPORTS,
        })
        .send()
        .expect("owner can create a proposal that is later cancelled");

    program
        .request()
        .accounts(solana_treasury_vault::accounts::CancelWithdrawal {
            owner: owner.pubkey(),
            treasury,
            proposal: cancelled_proposal,
        })
        .args(solana_treasury_vault::instruction::CancelWithdrawal {})
        .send()
        .expect("owner can cancel an unexecuted proposal");

    let cancelled_approval = program
        .request()
        .signer(&approver)
        .accounts(solana_treasury_vault::accounts::ApproveWithdrawal {
            approver: approver.pubkey(),
            treasury,
            proposal: cancelled_proposal,
        })
        .args(solana_treasury_vault::instruction::ApproveWithdrawal {})
        .send();
    assert!(
        cancelled_approval.is_err(),
        "a cancelled proposal must not be approved"
    );

    let cancelled_execution = program
        .request()
        .accounts(solana_treasury_vault::accounts::ExecuteWithdrawal {
            owner: owner.pubkey(),
            treasury,
            proposal: cancelled_proposal,
            recipient: owner.pubkey(),
        })
        .args(solana_treasury_vault::instruction::ExecuteWithdrawal {})
        .send();
    assert!(
        cancelled_execution.is_err(),
        "a cancelled proposal must not be executed"
    );
}
