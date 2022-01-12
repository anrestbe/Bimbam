use crate::type_engine::TypeId;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct ConcurrentSlab<T> {
    inner: RwLock<Vec<T>>,
}

impl<T> ConcurrentSlab<T>
where
    T: Clone + PartialEq,
{
    pub fn insert(&self, value: T) -> TypeId {
        let mut inner = self.inner.write().unwrap();
        let ret = inner.len();
        inner.push(value);
        TypeId(ret)
    }

    pub fn get(&self, index: TypeId) -> T {
        let inner = self.inner.read().unwrap();
        inner[index.0].clone()
    }

    pub fn replace(&self, index: TypeId, prev_value: &T, new_value: T) -> Option<T> {
        let mut inner = self.inner.write().unwrap();
        let actual_prev_value = &inner[index.0];
        if actual_prev_value != prev_value {
            return Some(actual_prev_value.clone());
        }
        inner[index.0] = new_value;
        None
    }
}
