pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] = elrond_wasm::hex_literal::hex!(
    "000000000000000000010000000000000000000000000000000000000002ffff"
);
pub const ISSUE_COST: u64 = 50_000_000_000_000_000;
pub const OWNER_EGLD_BALANCE: u64 = 100_000_000_000_000_000;
pub const USER_EGLD_BALANCE: u64 = 1_000_000_000;
pub const COLLECTION_ID: &[u8] = b"myCollection";

pub const FIRST_BRAND_ID: &[u8] = b"FirstBrand";
pub const FIRST_MEDIA_TYPE: &[u8] = b"png";
pub const FIRST_MAX_NFTS: usize = 10;
pub const FIRST_MINT_START_EPOCH: u64 = 3;
pub const FIRST_MINT_PRICE_TOKEN_ID: &[u8] = b"EGLD";
pub const FIRST_MINT_PRICE_AMOUNT: u64 = 1_000;
pub const FIRST_TOKEN_DISPLAY_NAME: &[u8] = b"FirstToken";
pub const FIRST_TOKEN_TICKER: &[u8] = b"FIRST";
pub const FIRST_TOKEN_ID: &[u8] = b"FIRST-000000";
pub const FIRST_TAGS: &[&[u8]] = &[b"funny", b"sad", b"memes"];

pub const SECOND_BRAND_ID: &[u8] = b"SecondBrand";
pub const SECOND_MEDIA_TYPE: &[u8] = b"mp3";
pub const SECOND_MAX_NFTS: usize = 15;
pub const SECOND_MINT_START_EPOCH: u64 = 5;
pub const SECOND_MINT_PRICE_TOKEN_ID: &[u8] = b"EGLD";
pub const SECOND_MINT_PRICE_AMOUNT: u64 = 100_000;
pub const SECOND_TOKEN_DISPLAY_NAME: &[u8] = b"SecondToken";
pub const SECOND_TOKEN_TICKER: &[u8] = b"SECOND";
pub const SECOND_TOKEN_ID: &[u8] = b"SECOND-111111";
pub const SECOND_TAGS: &[&[u8]] = &[b"random", b"good", b"best"];
