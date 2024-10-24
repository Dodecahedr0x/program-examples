mod mint;

use mint::*;

use create_token_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&create_token_api::ID, program_id, data)?;

    match ix {
        SteelInstruction::Mint => process_mint(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
