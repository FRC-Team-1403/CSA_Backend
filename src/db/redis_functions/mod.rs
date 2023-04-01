use crate::ram::ENV;
use redis::{pipe, Client, Commands, ConnectionLike};
use std::thread;

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
