use crate::ram::ENV;
use log::error;
use redis::{pipe, Client, Commands, Connection};
use std::thread;

pub struct RedisDb {
    con: Connection,
}

impl RedisDb {
    fn get_redis_data(key: String) -> redis::RedisResult<String> {
        let client = Client::open(ENV.redis_key.clone())?;
        let mut con = client.get_connection()?;

        con.get(key)
    }

    fn set_value(key: String, value: String) -> redis::RedisResult<bool> {
        let client = Client::open(ENV.redis_key.clone())?;
        let mut con = client.get_connection()?;
        con.set(key, value)
    }

    fn set_list(key: &str, values: Vec<&str>) -> redis::RedisResult<bool> {
        let redis = ENV.redis_key.clone();
        let client = Client::open(redis).unwrap();
        let mut con = client.get_connection().unwrap();
        let mut pipe = pipe();
        for value in values {
            pipe.rpush(key, value);
        }
        pipe.query(&mut con)
    }

    pub fn new() -> Option<Self> {
        let mut error: u8 = 0;
        loop {
            let con = Client::open(ENV.redis_key.clone());
            if let Ok(con) = con {
                if let Ok(con) = con.get_connection() {
                    return Some(Self { con });
                }
                error!("FAILED WHILE CONNECTING REDIS, RETRYING {error}")
            } else {
                error!("FAILED WHILE STARTING REDIS CLIENT, , RETRYING {error}")
            }
            if error > 120 {
                return None;
            }
            error += 1;
        }
    }

    pub fn set_team(&mut self) {
        todo!()
    }
}

pub fn clear() {
    thread::spawn(|| {
        let mut con = Client::open(ENV.redis_key.clone())
            .unwrap()
            .get_connection()
            .unwrap();
        let check: redis::RedisResult<()> = redis::cmd("FLUSHDB").query(&mut con);
        check.unwrap()
    });
}
