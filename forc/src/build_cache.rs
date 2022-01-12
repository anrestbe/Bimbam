#![allow(warnings)]
struct BuildCache {}

impl BuildCache {
    fn init() {
        // check if ~/.forc/build_cache exists
        // if not, create it
        todo!()
    }

    fn insert_item(key: u128, value: String) -> Result<(), ()> {
        todo!()
    }

    fn get(key: u128) -> Option<String> {
        todo!()
    }
}
