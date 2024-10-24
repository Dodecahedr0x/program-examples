#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod error {
    use steel::*;
    #[repr(u32)]
    pub enum CreateTokenError {
        #[error("Failed to derive metadata key")]
        MetadataDerivation = 0,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CreateTokenError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "MetadataDerivation")
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl std::error::Error for CreateTokenError {}
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for CreateTokenError {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                CreateTokenError::MetadataDerivation {} => {
                    __formatter.write_str("Failed to derive metadata key")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CreateTokenError {
        #[inline]
        fn clone(&self) -> CreateTokenError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CreateTokenError {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CreateTokenError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CreateTokenError {
        #[inline]
        fn eq(&self, other: &CreateTokenError) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CreateTokenError {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl From<CreateTokenError> for u32 {
        #[inline]
        fn from(enum_value: CreateTokenError) -> Self {
            enum_value as Self
        }
    }
    impl From<CreateTokenError> for solana_program::program_error::ProgramError {
        fn from(e: CreateTokenError) -> Self {
            solana_program::program_error::ProgramError::Custom(e as u32)
        }
    }
}
pub mod instruction {
    use steel::*;
    #[repr(u8)]
    pub enum SteelInstruction {
        Mint = 0,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SteelInstruction {
        #[inline]
        fn clone(&self) -> SteelInstruction {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SteelInstruction {}
    #[automatically_derived]
    impl ::core::fmt::Debug for SteelInstruction {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Mint")
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SteelInstruction {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SteelInstruction {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SteelInstruction {
        #[inline]
        fn eq(&self, other: &SteelInstruction) -> bool {
            true
        }
    }
    impl ::num_enum::TryFromPrimitive for SteelInstruction {
        type Primitive = u8;
        type Error = ::num_enum::TryFromPrimitiveError<Self>;
        const NAME: &'static str = "SteelInstruction";
        fn try_from_primitive(
            number: Self::Primitive,
        ) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>> {
            #![allow(non_upper_case_globals)]
            const Mint__num_enum_0__: u8 = 0;
            #[deny(unreachable_patterns)]
            match number {
                Mint__num_enum_0__ => ::core::result::Result::Ok(Self::Mint),
                #[allow(unreachable_patterns)]
                _ => {
                    ::core::result::Result::Err(
                        ::num_enum::TryFromPrimitiveError::<Self>::new(number),
                    )
                }
            }
        }
    }
    impl ::core::convert::TryFrom<u8> for SteelInstruction {
        type Error = ::num_enum::TryFromPrimitiveError<Self>;
        #[inline]
        fn try_from(
            number: u8,
        ) -> ::core::result::Result<Self, ::num_enum::TryFromPrimitiveError<Self>> {
            ::num_enum::TryFromPrimitive::try_from_primitive(number)
        }
    }
    #[doc(hidden)]
    impl ::num_enum::CannotDeriveBothFromPrimitiveAndTryFromPrimitive
    for SteelInstruction {}
    #[repr(C)]
    pub struct Mint {
        pub amount: u64,
        pub decimals: u8,
        pub name: [u8; 32],
        pub symbol: [u8; 23],
        pub uri: [u8; 256],
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Mint {
        #[inline]
        fn clone(&self) -> Mint {
            let _: ::core::clone::AssertParamIsClone<u64>;
            let _: ::core::clone::AssertParamIsClone<u8>;
            let _: ::core::clone::AssertParamIsClone<[u8; 32]>;
            let _: ::core::clone::AssertParamIsClone<[u8; 23]>;
            let _: ::core::clone::AssertParamIsClone<[u8; 256]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Mint {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Mint {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Mint",
                "amount",
                &self.amount,
                "decimals",
                &self.decimals,
                "name",
                &self.name,
                "symbol",
                &self.symbol,
                "uri",
                &&self.uri,
            )
        }
    }
    const _: fn() = || {
        #[doc(hidden)]
        struct TypeWithoutPadding(
            [u8; ::core::mem::size_of::<u64>() + ::core::mem::size_of::<u8>()
                + ::core::mem::size_of::<[u8; 32]>() + ::core::mem::size_of::<[u8; 23]>()
                + ::core::mem::size_of::<[u8; 256]>()],
        );
        let _ = ::core::mem::transmute::<Mint, TypeWithoutPadding>;
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<u64>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<u8>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<[u8; 32]>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<[u8; 23]>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<[u8; 256]>();
        }
    };
    unsafe impl ::bytemuck::Pod for Mint {}
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<u64>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<u8>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<[u8; 32]>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<[u8; 23]>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<[u8; 256]>();
        }
    };
    unsafe impl ::bytemuck::Zeroable for Mint {}
    impl Mint {
        pub fn try_from_bytes(
            data: &[u8],
        ) -> Result<&Self, solana_program::program_error::ProgramError> {
            bytemuck::try_from_bytes::<Self>(data)
                .or(
                    Err(
                        solana_program::program_error::ProgramError::InvalidInstructionData,
                    ),
                )
        }
    }
    impl ::steel::Discriminator for Mint {
        fn discriminator() -> u8 {
            SteelInstruction::Mint as u8
        }
    }
    impl Mint {
        pub fn to_bytes(&self) -> Vec<u8> {
            [[SteelInstruction::Mint as u8].to_vec(), bytemuck::bytes_of(self).to_vec()]
                .concat()
        }
    }
}
pub mod sdk {
    use mpl_token_metadata::accounts::Metadata;
    use spl_associated_token_account::get_associated_token_address;
    use steel::*;
    use sysvar::rent;
    use crate::prelude::*;
    pub fn array_to_string(array: &[u8]) -> String {
        let end = array.iter().position(|&x| x == 0).unwrap_or(array.len());
        String::from_utf8_lossy(&array[..end]).to_string()
    }
    pub fn string_to_fixed_array<const N: usize>(input: String) -> [u8; N] {
        let mut array = [0u8; N];
        let bytes = input.as_bytes();
        let len = if bytes.len() < N { bytes.len() } else { N };
        array[..len].copy_from_slice(&bytes[..len]);
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
            accounts: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(signer, true),
                    AccountMeta::new(Metadata::find_pda(&mint_key).0, false),
                    AccountMeta::new(mint_key, true),
                    AccountMeta::new(
                        get_associated_token_address(&signer, &mint_key),
                        false,
                    ),
                    AccountMeta::new_readonly(mpl_token_metadata::ID, false),
                    AccountMeta::new_readonly(spl_associated_token_account::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                    AccountMeta::new_readonly(rent::ID, false),
                ]),
            ),
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
}
pub mod prelude {
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
}
use steel::*;
/// The const program ID.
pub const ID: ::solana_program::pubkey::Pubkey = ::solana_program::pubkey::Pubkey::new_from_array([
    14u8,
    161u8,
    195u8,
    213u8,
    189u8,
    197u8,
    225u8,
    57u8,
    73u8,
    73u8,
    0u8,
    162u8,
    188u8,
    36u8,
    22u8,
    90u8,
    199u8,
    90u8,
    176u8,
    34u8,
    202u8,
    240u8,
    163u8,
    150u8,
    83u8,
    155u8,
    144u8,
    232u8,
    92u8,
    16u8,
    155u8,
    144u8,
]);
/// Returns `true` if given pubkey is the program ID.
pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID.
pub const fn id() -> ::solana_program::pubkey::Pubkey {
    ID
}
