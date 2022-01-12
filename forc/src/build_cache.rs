use anyhow::{anyhow, bail, Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all, File, OpenOptions},
    io::{BufReader, Read, Write},
    path::PathBuf,
};
use sway_utils::constants;

struct BuildCache {}

impl BuildCache {
    fn get_default_cache_dir() -> Result<PathBuf> {
        let home_dir = match home_dir() {
            None => return Err(anyhow!("Couldn't find home directory (`~/`)")),
            Some(p) => p.to_str().unwrap().to_owned(),
        };
        Ok(PathBuf::from(format!(
            "{}/{}/{}",
            home_dir,
            constants::FORC_DEPENDENCIES_DIRECTORY,
            constants::FORC_CACHE_DIRECTORY,
        )))
    }
    fn init() -> Result<Self> {
        let cache_dir = Self::get_default_cache_dir()?;
        // check if ~/.forc/build_cache exists
        // if not, create it
        create_dir_all(&cache_dir)?;
        Ok(BuildCache {})
    }

    /// Given a cache key `K`, check if it exists in the cache. If it does,
    /// return that value. If it does not, then this is a cache miss. Execute
    /// function `F` to get result `R` and store result `R` in the cache.
    pub fn cached<'a, F, R, K>(&mut self, key: K, func: F) -> Result<R>
    where
        F: Fn() -> R,
        R: for<'de> Deserialize<'de> + Serialize + Clone,
        K: BuildCacheKey,
    {
        if let Some(result) = self.get(&key)? {
            return Ok(result);
        }
        let result = func();
        self.insert(&key, result.clone())?;
        Ok(result)
    }

    fn get<'a, K, R>(&self, key: &K) -> Result<Option<R>>
    where
        K: BuildCacheKey,
        R: for<'de> Deserialize<'de>,
    {
        let cache_dir = Self::get_default_cache_dir()?;
        let listing = fs::read_dir(&cache_dir)?.collect::<std::result::Result<Vec<_>, _>>()?;
        let cache_key: Key = key.to_key();
        let cached_result = listing
            .into_iter()
            .find(|path| &path.file_name().to_string_lossy().to_string() == &cache_key);
        match cached_result {
            Some(o) => {
                let f = File::open(o.path())?;
                let mut reader = BufReader::new(f);
                let mut buffer = Vec::new();

                reader.read_to_end(&mut buffer)?;

                // deserialize bytes into R
                Ok(Some(bincode::deserialize_from(&buffer[..])?))
            }
            None => return Ok(None),
        }
    }

    fn insert<'a, K, R>(&mut self, key: &K, value: R) -> Result<()>
    where
        K: BuildCacheKey,
        R: Serialize,
    {
        let key = key.to_key();
        let mut path_to_cache_entry = Self::get_default_cache_dir()?;
        path_to_cache_entry.push(key);
        let mut file = OpenOptions::new().write(true).open(path_to_cache_entry)?;

        file.write_all(&bincode::serialize(&value)?);
        Ok(())
    }
}

trait BuildCacheKey {
    fn to_key(&self) -> Key;
}

type Key = String;
