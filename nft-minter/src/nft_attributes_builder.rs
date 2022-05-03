elrond_wasm::imports!();

use crate::{
    common_storage::{BrandId, CollectionHash, GenericAttributes, MediaType, Uri},
    unique_id_mapper::UniqueId,
};

static TAGS_PREFIX: &[u8] = b"tags:";
static TAG_SEPARATOR: &[u8] = b",";
static ATTRIBUTES_SEPARATOR: &[u8] = b";";
static SLASH: &[u8] = b"/";
static DOT: &[u8] = b".";

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
        collection_hash: &CollectionHash<Self::Api>,
        brand_id: &BrandId<Self::Api>,
        nft_id: UniqueId,
    ) -> GenericAttributes<Self::Api> {
        let mut attributes = self.build_attributes_metadata_part(collection_hash, nft_id);
        let tags_attributes = self.build_attributes_tags_part(brand_id);
        if !tags_attributes.is_empty() {
            attributes.append_bytes(ATTRIBUTES_SEPARATOR);
            attributes.append(&tags_attributes);
        }

        attributes
    }

    fn build_attributes_metadata_part(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
        nft_id: UniqueId,
    ) -> GenericAttributes<Self::Api> {
        sc_format!(
            "metadata:{}{}{}.json",
            collection_hash.as_managed_buffer(),
            SLASH,
            nft_id
        )
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
        collection_hash: &CollectionHash<Self::Api>,
        nft_id: UniqueId,
        media_type: &MediaType<Self::Api>,
    ) -> Uri<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}{}{}{}{}",
            collection_hash.as_managed_buffer(),
            SLASH,
            nft_id,
            DOT,
            media_type
        )
    }

    fn build_nft_json_file_uri(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
        nft_id: UniqueId,
    ) -> Uri<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}{}{}.json",
            collection_hash.as_managed_buffer(),
            SLASH,
            nft_id,
        )
    }

    fn build_collection_json_file_uri(
        &self,
        collection_hash: &CollectionHash<Self::Api>,
    ) -> Uri<Self::Api> {
        sc_format!(
            "https://ipfs.io/ipfs/{}/collection.json",
            collection_hash.as_managed_buffer(),
        )
    }

    fn is_supported_media_type(&self, media_type: &MediaType<Self::Api>) -> bool {
        let media_type_len = media_type.len();
        if media_type_len > MAX_MEDIA_TYPE_LEN {
            return false;
        }

        let mut media_static_buffer = [0u8; MAX_MEDIA_TYPE_LEN];
        let slice = &mut media_static_buffer[..media_type_len];
        let _ = media_type.load_slice(0, slice);

        // clippy is wrong, using `slice` directly causes an error
        #[allow(clippy::redundant_slicing)]
        SUPPORTED_MEDIA_TYPES.contains(&&slice[..])
    }
}
