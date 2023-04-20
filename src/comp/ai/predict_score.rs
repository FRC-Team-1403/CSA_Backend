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
    year: 5.8,
    guess: 0.1,
    remove: 2.82,
};

impl Ai {
    pub fn predict_match_score(data: &YearAround, team: &u16) -> f32 {
        let year_avg_guess = if let Some(team_data) = Self::get_lock_find(team) {
            (Self::geometic_mean(&data.points.graph)
                + (Self::geometic_mean(&team_data.points.graph) * SCORE_AI.year))
                / (1.0 + SCORE_AI.year)
        } else {
            Self::geometic_mean(&data.points.graph)
        };
        let ai_guess =
            Self::line_point_regression(&data.points.graph, Math::Score) * SCORE_AI.guess;
        ((ai_guess + year_avg_guess) / (1.0 + SCORE_AI.guess)) / SCORE_AI.remove
    }
}
