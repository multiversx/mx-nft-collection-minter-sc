elrond_wasm::imports!();

use elrond_wasm::{
    api::{CallTypeApi, StorageMapperApi},
    storage::StorageKey,
    storage_set,
};

pub type UniqueId = usize;
const EMPTY_ENTRY: UniqueId = 0;

/// Holds the values from 1 to N with as little storage interaction as possible
/// If Mapper[i] = i, then it stores nothing, i.e. "0"
/// If Mapper[i] is equal to another value, then it stores the value
pub struct UniqueIdMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    base_key: StorageKey<SA>,
    vec_mapper: VecMapper<SA, UniqueId>,
}

impl<SA> StorageMapper<SA> for UniqueIdMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            base_key: base_key.clone(),
            vec_mapper: VecMapper::new(base_key),
        }
    }
}

impl<SA> UniqueIdMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    /// Manually overwrite VecMapper's len value
    pub fn set_initial_len(&mut self, len: usize) {
        if !self.vec_mapper.is_empty() {
            SA::error_api_impl().signal_error(b"len already set");
        }

        self.set_internal_mapper_len(len);
    }

    pub fn len(&self) -> usize {
        self.vec_mapper.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec_mapper.is_empty()
    }

    /// if there is no stored value, it means we have to return the index as the value
    pub fn get(&self, index: usize) -> UniqueId {
        let nonce: UniqueId = self.vec_mapper.get(index);
        if nonce == EMPTY_ENTRY {
            index
        } else {
            nonce
        }
    }

    pub fn get_and_swap_remove(&mut self, index: usize) -> UniqueId {
        let last_item_index = self.len();
        let last_item = self.get(last_item_index);

        let current_item = if index != last_item_index {
            let item_at_index = self.get(index);
            self.vec_mapper.set(index, &last_item);

            item_at_index
        } else {
            last_item
        };

        self.set_internal_mapper_len(last_item_index - 1);

        current_item
    }

    pub fn clear_len(&mut self) {
        self.set_internal_mapper_len(0);
    }

    // Manually sets the internal VecMapper's len value
    fn set_internal_mapper_len(&mut self, new_len: usize) {
        let mut len_key = self.base_key.clone();
        len_key.append_bytes(&b".len"[..]);
        storage_set(len_key.as_ref(), &new_len);
    }
}
