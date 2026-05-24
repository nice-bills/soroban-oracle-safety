use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AdapterError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAdmin = 3,
    InvalidConfig = 4,
}

impl From<oracle_storage::AdapterError> for AdapterError {
    fn from(e: oracle_storage::AdapterError) -> Self {
        match e {
            oracle_storage::AdapterError::AlreadyInitialized => Self::AlreadyInitialized,
            oracle_storage::AdapterError::NotInitialized => Self::NotInitialized,
            oracle_storage::AdapterError::NotAdmin => Self::NotAdmin,
            oracle_storage::AdapterError::InvalidConfig => Self::InvalidConfig,
        }
    }
}
