use dotenv;
use std::env;
use std::path::{Path};

use redis::{pipe, Client, Commands};

fn get_redis_data(key: String) -> redis::RedisResult<String> {
    let my_path = env::home_dir().map(|a| a.join("/.env")).unwrap();
    dotenv::from_path(my_path.as_path()).expect("No .env file detected");
    let redis = dotenv::var("redis").unwrap();

    let client = Client::open(redis)?;
    let mut con = client.get_connection()?;

    con.get(key)
}

fn set_value(key: String, value: String) -> redis::RedisResult<bool> {
    let my_path = env::home_dir().map(|a| a.join("/.env")).unwrap();
    dotenv::from_path(my_path.as_path()).expect("No .env file detected");
    let redis = dotenv::var("redis").unwrap();

    let client = Client::open(redis)?;
    let mut con = client.get_connection()?;
    con.set(key, value)
}

fn set_list(key: &str, values: Vec<&str>) -> redis::RedisResult<bool> {
    let my_path = env::home_dir().map(|a| a.join("/.env")).unwrap();
    dotenv::from_path(my_path.as_path()).expect("No .env file detected");
    let redis = dotenv::var("redis").unwrap();

    let client = Client::open(redis).unwrap();
    let mut con = client.get_connection().unwrap();
    let mut pipe = pipe();
    for value in values {
        pipe.rpush(key, value);
    }
    pipe.query(&mut con)
}
