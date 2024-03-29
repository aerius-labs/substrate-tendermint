pub(crate) type Id = u64;
pub(crate) type Signature = u64;
pub(crate) type Hash = &'static str;
pub(crate) type BlockNumber = u64;

pub const GENESIS_HASH: &str = "genesis";
const NULL_HASH: &str = "NULL";

pub(crate) mod chain;
pub(crate) mod environment;
pub(crate) mod network;
