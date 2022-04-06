elrond_wasm::imports!();

use crate::{
    common_storage::{BrandId, GenericAttributes, MediaType, Uri},
    unique_id_mapper::UniqueId,
};
use core::convert::TryInto;

static BASE_URI: &[u8] = b"https://ipfs.io/ipfs";
static COLLECTION_INFO_FILE_NAME: &[u8] = b"collection";
static JSON_FILE_EXTENSION: &[u8] = b"json";
static METADATA_PREFIX: &[u8] = b"metadata:";
static TAGS_PREFIX: &[u8] = b"tags:";

static SLASH: &[u8] = b"/";
static DOT: &[u8] = b".";
static TAG_SEPARATOR: &[u8] = b",";
static ATTRIBUTES_SEPARATOR: &[u8] = b";";

static SUPPORTED_MEDIA_TYPES: &[&[u8]] = &[
    b"png",
    b"jpeg",
    b"jpg",
    b"gif",
    b"acc",
    b"flac",
    b"m4a",
    b"mp3",
    b"wav",
    b"mov",
    b"quicktime",
    b"mp4",
    b"webm",
];
const MAX_MEDIA_TYPE_LEN: usize = 9;

#[elrond_wasm::module]
pub trait NftAttributesBuilderModule: crate::common_storage::CommonStorageModule {
    fn build_nft_attributes(
        &self,
        brand_id: &BrandId<Self::Api>,
        nft_id: UniqueId,
    ) -> GenericAttributes<Self::Api> {
        let mut attributes = self.build_attributes_metadata_part(nft_id);
        let tags_attributes = self.build_attributes_tags_part(brand_id);
        if !tags_attributes.is_empty() {
            attributes.append_bytes(ATTRIBUTES_SEPARATOR);
            attributes.append(&tags_attributes);
        }

        attributes
    }

    fn build_attributes_metadata_part(&self, nft_id: UniqueId) -> GenericAttributes<Self::Api> {
        let collection_id = self.parent_collection_id().get();
        let id_ascii = self.decimal_to_ascii(nft_id as u32);

        let mut metadata = GenericAttributes::new_from_bytes(METADATA_PREFIX);
        metadata.append(&collection_id);
        metadata.append_bytes(SLASH);
        metadata.append(&id_ascii);
        metadata.append_bytes(DOT);
        metadata.append_bytes(JSON_FILE_EXTENSION);

        metadata
    }

    fn build_attributes_tags_part(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> GenericAttributes<Self::Api> {
        let all_tags = self.tags_for_brand(brand_id).get();
        let tags_len = all_tags.len();
        if tags_len == 0 {
            return GenericAttributes::new();
        }

        let mut tags_attributes = GenericAttributes::new_from_bytes(TAGS_PREFIX);
        for i in 0..tags_len - 1 {
            let tag = all_tags.get(i);
            tags_attributes.append(&tag);
            tags_attributes.append_bytes(TAG_SEPARATOR);
        }

        let last_tag = all_tags.get(tags_len - 1);
        tags_attributes.append(&last_tag);

        tags_attributes
    }

    fn build_nft_main_file_uri(
        &self,
        nft_id: UniqueId,
        media_type: &MediaType<Self::Api>,
    ) -> Uri<Self::Api> {
        let mut uri = self.build_base_uri_for_id(nft_id);
        uri.append(media_type);

        uri
    }

    fn build_nft_json_file_uri(&self, nft_id: UniqueId) -> Uri<Self::Api> {
        let mut uri = self.build_base_uri_for_id(nft_id);
        uri.append_bytes(JSON_FILE_EXTENSION);

        uri
    }

    fn build_collection_json_file_uri(&self) -> Uri<Self::Api> {
        let mut uri = self.build_base_collection_uri();
        uri.append_bytes(COLLECTION_INFO_FILE_NAME);
        uri.append_bytes(DOT);
        uri.append_bytes(JSON_FILE_EXTENSION);

        uri
    }

    fn build_base_uri_for_id(&self, nft_id: UniqueId) -> Uri<Self::Api> {
        let id_ascii = self.decimal_to_ascii(nft_id as u32);

        let mut uri = self.build_base_collection_uri();
        uri.append(&id_ascii);
        uri.append_bytes(DOT);

        uri
    }

    fn build_base_collection_uri(&self) -> Uri<Self::Api> {
        let collection_id = self.parent_collection_id().get();

        let mut uri = Uri::new_from_bytes(BASE_URI);
        uri.append_bytes(SLASH);
        uri.append(&collection_id);
        uri.append_bytes(SLASH);

        uri
    }

    fn is_supported_media_type(&self, media_type: &MediaType<Self::Api>) -> bool {
        let media_type_len = media_type.len();
        if media_type_len > MAX_MEDIA_TYPE_LEN {
            return false;
        }

        let mut media_static_buffer = [0u8; MAX_MEDIA_TYPE_LEN];
        let slice = &mut media_static_buffer[..media_type_len];
        let _ = media_type.load_slice(0, slice);

        SUPPORTED_MEDIA_TYPES.contains(&&slice[..])
    }

    fn decimal_to_ascii(&self, mut number: u32) -> ManagedBuffer {
        const MAX_NUMBER_CHARACTERS: usize = 10;
        const ZERO_ASCII: u8 = b'0';

        let mut as_ascii = [0u8; MAX_NUMBER_CHARACTERS];
        let mut nr_chars = 0;

        loop {
            unsafe {
                let reminder: u8 = (number % 10).try_into().unwrap_unchecked();
                number /= 10;

                as_ascii[nr_chars] = ZERO_ASCII + reminder;
                nr_chars += 1;
            }

            if number == 0 {
                break;
            }
        }

        let slice = &mut as_ascii[..nr_chars];
        slice.reverse();

        ManagedBuffer::new_from_bytes(slice)
    }
}
