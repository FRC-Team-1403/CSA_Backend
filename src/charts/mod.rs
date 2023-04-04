mod http;

use rayon::prelude::*;

pub fn populate() {
    let teams_to_track: Vec<u16> = vec![1403, 1923, 2595, 1623];
    teams_to_track.par_iter().for_each(|team| {});
}
