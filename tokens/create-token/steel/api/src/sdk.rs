use mpl_token_metadata::accounts::Metadata;
use spl_associated_token_account::get_associated_token_address;
use steel::*;
use sysvar::rent;

use crate::prelude::*;

pub fn array_to_string(array: &[u8]) -> String {
    let end = array.iter().position(|&x| x == 0).unwrap_or(array.len()); // Find the first zero byte (padding)
    String::from_utf8_lossy(&array[..end]).to_string() // Convert bytes up to the first zero to a String
}

pub fn string_to_fixed_array<const N: usize>(input: String) -> [u8; N] {
    let mut array = [0u8; N]; // Create an array of zeros
    let bytes = input.as_bytes(); // Get the byte representation of the string

    let len = if bytes.len() < N { bytes.len() } else { N }; // Determine the length to copy

    array[..len].copy_from_slice(&bytes[..len]); // Copy the slice into the array
    array
}

pub fn mint(
    signer: Pubkey,
    mint_key: Pubkey,
    amount: u64,
    decimals: u8,
    name: String,
    symbol: String,
    uri: String,
) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(Metadata::find_pda(&mint_key).0, false),
            AccountMeta::new(mint_key, true),
            AccountMeta::new(get_associated_token_address(&signer, &mint_key), false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(rent::ID, false),
        ],
        data: Mint {
            amount,
            decimals,
            name: string_to_fixed_array(name),
            symbol: string_to_fixed_array(symbol),
            uri: string_to_fixed_array(uri),
        }
        .to_bytes(),
    }
}
