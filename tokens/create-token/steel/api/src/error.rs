use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum CreateTokenError {
    #[error("Failed to derive metadata key")]
    MetadataDerivation = 0,
}

error!(CreateTokenError);
