use crate::comp::ai::{Ai, Math};
use crate::comp::avg::math::YearAround;

pub struct ScoreAi {
    pub plr: f32,
    year: f32,
    guess: f32,
    remove: f32,
}

pub const SCORE_AI: ScoreAi = ScoreAi {
    plr: 0.0001,
    year: 4.8,
    guess: 0.1,
    remove: 0.00,
};

impl Ai {
    pub fn predict_match_score(data: &YearAround, team: &u16) -> f32 {
        let year_avg_guess = if let Some(team_data) = Self::get_lock_find(team) {
            (data.points.avg + (team_data.points.avg * SCORE_AI.year)) / (1.0 + SCORE_AI.year)
        } else {
            data.points.avg
        };
        let ai_guess =
            Self::line_point_regression(&data.points.graph, Math::Score) * SCORE_AI.guess;
        (ai_guess + year_avg_guess) / (1.0 + SCORE_AI.guess + SCORE_AI.remove)
    }
}
