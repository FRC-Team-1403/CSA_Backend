use crate::constant::REDIS;
use redis::{pipe, Client, Commands};

fn get_redis_data(key: String) -> redis::RedisResult<String> {
    let client = Client::open(REDIS)?;
    let mut con = client.get_connection()?;

    con.get(key)
}

fn set_value(key: String, value: String) -> redis::RedisResult<bool> {
    let client = Client::open(REDIS)?;
    let mut con = client.get_connection()?;
    con.set(key, value)
}

fn set_list(key: &str, values: Vec<&str>) -> redis::RedisResult<bool> {
    let client = Client::open(REDIS).unwrap();
    let mut con = client.get_connection().unwrap();
    let mut pipe = pipe();
    for value in values {
        pipe.rpush(key, value);
    }
    pipe.query(&mut con)
}
