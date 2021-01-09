use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use serde::export::fmt::Display;
use serde::de::DeserializeOwned;
use std::error::Error;

pub static CACHE_I64: Lazy<Mutex<HashMap<String, i64>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    Mutex::new(m)
});

static CACHE_STRING: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    Mutex::new(m)
});

#[derive(Serialize, Deserialize)]
struct CacheItem<T> {
    #[serde(bound(serialize="T: Serialize", deserialize="T: DeserializeOwned"))]
    pub data: T,
    pub ts: i64,
    pub ttl: i64,
}

pub fn setex<T>(key: &str, value: T, ttl: i64) -> Result<(), Box<dyn Error>> where T: Serialize + DeserializeOwned {
    let item = CacheItem {
        data: value,
        ts: chrono::Utc::now().timestamp_millis(),
        ttl,
    };

    let data = serde_json::to_string(&item)?;
    let mut cc = CACHE_STRING.lock()?;
    cc.insert(key.to_owned(), data);

    Ok(())
}

pub fn set<T>(key: &str, value: T) -> Result<(), Box<dyn Error>> where T: Serialize + DeserializeOwned {
    setex(key, value, -1)
}

pub fn get<T>(key: &str) -> Option<T> where T: Serialize + DeserializeOwned {
    let cc = CACHE_STRING.lock().ok()?;
    let item_str = cc.get(&key.to_owned())?;

    let item: CacheItem<T> = serde_json::from_str(item_str.as_str()).ok()?;

    if item.ttl <= 0 {
        return Some(item.data);
    }

    let now = chrono::Utc::now().timestamp_millis();
    if now - item.ts <= item.ttl {
        return Some(item.data);
    }

    None
}


#[cfg(test)]
mod test {
    use crate::util::local_cache;
    use serde::{Serialize, Deserialize};


    #[test]
    fn test_i64() {
        if let Ok(mut cc) = local_cache::CACHE_I64.lock() {
            for i in 0..10000 {
                cc.insert(format!("key{}", i), i);
            }
        }

        if let Ok(mut cc) = local_cache::CACHE_I64.lock() {
            for i in 0..10000 {
                if let Some(v) = cc.get(&format!("key{}", i)) {
                    println!("{:?}", v);
                }
            }
        }
    }

    #[test]
    fn test() {
        local_cache::set("aa", 3);

        assert_eq!(local_cache::get::<i32>("aa"), Some(3));
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct UserTest {
        pub id: i64,
        pub username: String,
        pub password: Option<String>
    }

    #[test]
    fn test2() {
        let user = UserTest {
            id: 10,
            username: "111".to_string(),
            password: Some("asdf".to_owned())
        };

        local_cache::set("aa", user.clone());

        assert_eq!(local_cache::get::<UserTest>("aa"), Some(user));
    }
}

