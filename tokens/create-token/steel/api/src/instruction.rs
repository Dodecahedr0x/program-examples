use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum SteelInstruction {
    Mint = 0,
}

// Taken from link below, but enlarged to match struct padding
// https://github.com/metaplex-foundation/mpl-token-metadata/blob/d580a0b144aeac7972b73490cde55b4877351b63/programs/token-metadata/program/src/state/metadata.rs#L20
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mint {
    pub amount: u64,
    pub decimals: u8,
    pub name: [u8; 32],
    pub symbol: [u8; 23],
    pub uri: [u8; 256],
}

instruction!(SteelInstruction, Mint);
