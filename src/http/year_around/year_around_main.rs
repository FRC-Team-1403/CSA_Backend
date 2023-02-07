use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

use crate::db::firebase::Firebase;
use crate::http::year_around::fuctions::get;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;
use crate::http::year_around::math::YearAround;

pub struct YearData {
    pub cache: HashMap<u16, TeamYearAroundJsonParser>,
}
impl YearData {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn check_cache(mut self, json: TeamYearAroundJsonParser, team: &str) -> (Self, bool) {
        let loc: u16 = team.parse().unwrap();
        if let Some(compare) = self.cache.get(&loc) {
            if compare == &json {
                println!("Skipping {team}, The data is not updated");
                return (self, false);
            }
        }
        self.cache.insert(loc, json);
        (self, true)
    }
    pub async fn update(mut self, year_check: u16) -> Result<Self, Self> {
        let mut wait: Vec<JoinHandle<()>> = vec![];
        let teams: Vec<i16> = vec![
            613, 714, 747, 752, 816, 869, 896, 1089, 1228, 1257, 1279, 1403, 1626, 1647, 1672,
            1676, 1791, 1807, 1809, 1811, 1812, 1914, 1923, 1989, 2016, 2070, 2180, 2191, 2458,
            2495, 2554, 2577, 2590, 2600, 2720, 2722, 2729, 3142, 3151, 3314, 3340, 3515, 3637,
            4035, 4281, 4361, 4475, 4573, 4652, 4653, 4750, 5113, 5310, 5420, 5438, 5457, 5624,
            5666, 5684, 5732, 5895, 5992, 6015, 6016, 6203, 6226, 6860, 6897, 6921, 6943, 6945,
            7024, 7045, 7110, 7587, 7771, 7801, 7853, 7877, 8075, 8102, 8130, 8139, 8157, 8513,
            8588, 8628, 8630, 8704, 8706, 8707, 8714, 8721, 8771, 8801, 9015, 9064, 9100, 9116, 11,
            25, 41, 56, 75, 87, 102, 136, 193, 203, 204, 219, 223, 224, 265, 293, 303, 316, 555,
        ];
        let amount = teams.len() - 1;
        let mut good: bool = false;
        for loc in 0..amount + 1 {
            let team = teams[loc].to_string();
            let data: TeamYearAroundJsonParser = {
                let mut _failed: u8 = 0;
                loop {
                    let response = get(&team, year_check).await;
                    if let Ok(json) = response {
                        break json;
                    } else {
                        if _failed == 120 {
                            println!("failed to get data");
                            return Err(self);
                        }
                        _failed += 1
                    }
                }
            };
            let mut _allow: bool = false;
            (self, _allow) = self.check_cache(data.clone(), &team);
            if _allow {
                let year = YearAround::new(data).calculate(&team);
                let Ok(year) = year else {
                println!("failed to parse data");
                return Err(self);
            };
                //checking if data exists
                if year.rp.avg.is_none() {
                    println!("Advanced data unavailable for team {}", &team)
                }
                if year.points.lowest == 10000 {
                    println!("Data unavailable for {} , skipping...", &team)
                } else {
                    println!(
                        "Full data is found and is pushed to firstore for {}!\n\
            Amount Completed {}/{}",
                        &team, loc, amount
                    );
                    good = true;
                    wait.push(thread::spawn(move || {
                        Firebase::new(year)
                            .set_year(&team, year_check)
                            .expect("failed when writing data");
                    }));
                }
            }
        }
        if !good {
            return Err(self);
        }
        println!("waiting for jobs to finish");
        for thread in wait {
            if thread.join().is_err() {
                return Err(self);
            }
        }
        println!("done!");
        Ok(self)
    }
}
