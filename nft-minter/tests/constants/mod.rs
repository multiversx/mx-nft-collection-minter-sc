use nft_minter::nft_attributes_builder::COLLECTION_HASH_LEN;

pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] = elrond_wasm::hex_literal::hex!(
    "000000000000000000010000000000000000000000000000000000000002ffff"
);
pub const ISSUE_COST: u64 = 50_000_000_000_000_000;
pub const OWNER_EGLD_BALANCE: u64 = 150_000_000_000_000_000;
pub const USER_EGLD_BALANCE: u64 = 1_000_000_000;
pub const CATEGORY: &[u8] = b"VeryCoolNfts";
pub const EGLD_TOKEN_ID: &[u8] = b"EGLD";
pub const MAX_NFTS_PER_TX: usize = 2;

pub const FIRST_COLLECTION_HASH: &[u8; COLLECTION_HASH_LEN] =
    b"FirstCollection_______________________________";
pub const FIRST_BRAND_ID: &[u8] = b"FirstBrand";
pub const FIRST_MEDIA_TYPE: &[u8] = b"png";
pub const FIRST_MINT_START_TIMESTAMP: u64 = 100_000_000;
pub const FIRST_MINT_END_TIMESTAMP: u64 = 200_000_000;
pub const FIRST_MINT_PRICE_TOKEN_ID: &[u8] = EGLD_TOKEN_ID;
pub const FIRST_MINT_PRICE_AMOUNT: u64 = 1_000;
pub const FIRST_TOKEN_DISPLAY_NAME: &[u8] = b"FirstToken";
pub const FIRST_TOKEN_TICKER: &[u8] = b"FIRST";
pub const FIRST_TOKEN_ID: &[u8] = b"FIRST-000000";
pub const FIRST_TAGS: &[&[u8]] = &[b"funny", b"sad", b"memes"];
pub const FIRST_TIERS: &[&[u8]] = &[b"gold", b"silver", b"bronze"];
pub const FIRST_NFT_AMOUNTS: &[usize] = &[5, 10, 20];

pub const SECOND_COLLECTION_HASH: &[u8; COLLECTION_HASH_LEN] =
    b"SecondCollection______________________________";
pub const SECOND_BRAND_ID: &[u8] = b"SecondBrand";
pub const SECOND_MEDIA_TYPE: &[u8] = b"mp3";
pub const SECOND_MINT_START_TIMESTAMP: u64 = 200_000_000;
pub const SECOND_MINT_END_TIMESTAMP: u64 = u64::MAX;
pub const SECOND_MINT_PRICE_TOKEN_ID: &[u8] = EGLD_TOKEN_ID;
pub const SECOND_MINT_PRICE_AMOUNT: u64 = 100_000;
pub const SECOND_TOKEN_DISPLAY_NAME: &[u8] = b"SecondToken";
pub const SECOND_TOKEN_TICKER: &[u8] = b"SECOND";
pub const SECOND_TOKEN_ID: &[u8] = b"SECOND-111111";
pub const SECOND_TAGS: &[&[u8]] = &[b"random", b"good", b"best"];
pub const SECOND_TIERS: &[&[u8]] = &[b"gold", b"silver", b"bronze"];
pub const SECOND_NFT_AMOUNTS: &[usize] = &[10, 15, 25];

pub const THIRD_COLLECTION_HASH: &[u8; COLLECTION_HASH_LEN] =
    b"ThirdCollection_______________________________";
pub const THIRD_BRAND_ID: &[u8] = b"ThirdBrand";
