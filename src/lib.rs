use grid::grid_to_string;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use solver::try_to_solve;

mod grid;
mod solver;

const MAX_SIZE: usize = 10;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() < 2 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let width = instruction_data[0]; // can be stored in a u8, as max 10, 10 < 2^8-1
    let height = instruction_data[1]; // can be stored in a u8, as max 10, 10 < 2^8-1

    // first make "cheap" checks, better perf
    if width > MAX_SIZE as u8
        || height > MAX_SIZE as u8
        || instruction_data.len() < (2 + width * height) as usize
    {
        return Err(ProgramError::InvalidInstructionData);
    }

    // after validating, convert to usize, can't panic!, safe
    let width = width as usize;
    let height = height as usize;

    let grid = &instruction_data[2..];
    msg!("\n{}", grid_to_string(grid, width));

    let solution = try_to_solve(&grid, width, height);
    match solution {
        Some(solved_grid) => msg!("Solution found!,\n{}", grid_to_string(&solved_grid, width)),
        None => msg!("Looks like the grid is unsolvable!"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    const GRID_SAMPLES_PER_DIM: usize = 64;

    use super::*;
    use grid::random_grid;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signer::Signer,
        transaction::Transaction,
    };

    struct TestState {
        pub(crate) program_id: Pubkey,
        pub(crate) account_key: Pubkey,
        pub(crate) program: ProgramTest,
    }

    impl TestState {
        fn new(name: &str) -> TestState {
            let (program_id, account_key) = (Pubkey::new_unique(), Pubkey::new_unique());
            let program = ProgramTest::new(name, program_id, processor!(process_instruction));
            TestState {
                program_id,
                account_key,
                program,
            }
        }
    }

    #[tokio::test]
    async fn test_minesweeper_solver_10x10_1() {
        let mut state = TestState::new("solana_program0");

        let grid = [
            5, 5, 1, 2, 9, 1, 0, 2, 10, 2, 1, 0, 9, 2, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        state.program.add_account(
            state.account_key,
            Account {
                lamports: 1_000_000,
                data: Vec::with_capacity(0),
                owner: state.program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = state.program.start().await;

        let instruction = Instruction::new_with_bytes(
            state.program_id,
            &grid,
            vec![AccountMeta::new(state.account_key, false)],
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();
    }

    #[tokio::test]
    async fn test_minesweeper_solver_rand() {

        let mut state = TestState::new("solana_program0");

        let width_height_pairs: Vec<(usize, usize)> = (1..=MAX_SIZE).zip(1..=MAX_SIZE).collect();
        assert!(width_height_pairs.len() == MAX_SIZE); // just to make sure

        let grids: Vec<Vec<u8>> = width_height_pairs
            .into_iter()
            .map(|(w, h)| {
                let mut xs = Vec::with_capacity(GRID_SAMPLES_PER_DIM);

                for _ in 0..GRID_SAMPLES_PER_DIM {
                    let mut x = Vec::with_capacity(2 + w * h);
                    x.push(w as u8);
                    x.push(h as u8);
                    x.extend(&random_grid(w, h));
                    xs.push(x);
                }
                xs
            })
            .flatten()
            .collect(); // inefficent TLB wise, will opt later.

        state.program.add_account(
            state.account_key,
            Account {
                lamports: 1_000_000, // lamports, not lesslie lamport ;)
                data: Vec::with_capacity(0),
                owner:state.program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) =state.program.start().await;
        

        let transactions: Vec<Transaction> = grids
            .into_iter()
            .map(|grid| {
                let instruction = Instruction::new_with_bytes(
                    state.program_id,
                    &grid,
                    vec![AccountMeta::new(state.account_key, false)],
                );
                Transaction::new_signed_with_payer(
                    &[instruction],
                    Some(&payer.pubkey()),
                    &[&payer],
                    recent_blockhash,
                )
            })
            .collect();

        for tx in transactions.into_iter() {
            banks_client.process_transaction(tx).await.unwrap();
        }
    }
}
