use crate::comp::avg::math::YearAround;
use crate::db::firebase::get::get_avg;
use crate::ram::ENV;
use log::error;
use redis::{pipe, Client, Commands, Connection, RedisResult};
use std::thread;
use std::time::Duration;

pub struct RedisDb {
    pub con: Connection,
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
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn set_team(&mut self, team: &u16, kind: &str, value: Option<f32>) {
        let Some(value) = value else {
            return;
        };
        let mut error: u8 = 0;
        loop {
            let retry: RedisResult<()> = self.con.zadd(kind, team, value);
            if retry.is_ok() {
                return;
            }
            if error > 10 {
                error!("SKIPPING {team} DUE TO REDIS FAILURE");
                return;
            }
            error!(
                "FAILED WHILE SENDING REDIS DATA, {}, RETRYING {error}",
                retry.unwrap_err()
            );
            error += 1;
            thread::sleep(Duration::from_secs(1));
        }
    }
    pub fn send_avg_redis(&mut self, team: &u16, data: &YearAround) {
        self.set_team(team, "TeamAuto", data.auto.avg);
        self.set_team(team, "TeamPoints", Some(data.points.avg));
        self.set_team(team, "TeamPenalty", data.pen.avg);
        self.set_team(team, "RankingPoints", data.rp.avg);
        self.set_team(team, "Deviation", Some(data.deviation));
        self.set_team(team, "WinRatio", Some(data.win_rato));
        self.set_team(team, "EkamAi", Some(data.ekam_ai));
        self.set_team(team, "auto_game_pieces", data.auto_game_pieces.avg);
        self.set_team(team, "auto_game_points", data.auto_game_points.avg);
        self.set_team(team, "telop_game_pieces", data.telop_game_pieces.avg);
        self.set_team(team, "telop_game_points", data.telop_game_points.avg);
        // let app_data = get_avg(team);
        // match app_data {
        //     Ok(app_data) => {
        //         self.set_team(team, "TotalContributed", Some(app_data.total));
        //         self.set_team(team, "AutoContributed", Some(app_data.auto));
        //         self.set_team(team, "TeleopContributed", Some(app_data.teleop));
        //     }
        //     Err(e) => {
        //         error!("Error when getting data from firestore {e}")
        //     }
        // }
    }
}

pub fn clear() {
    thread::spawn(|| {
        let mut con = Client::open(ENV.redis_key.clone())
            .unwrap()
            .get_connection()
            .unwrap();
        let check: RedisResult<()> = redis::cmd("FLUSHDB").query(&mut con);
        check.unwrap()
    });
}
