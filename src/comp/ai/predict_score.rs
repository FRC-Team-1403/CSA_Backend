use crate::comp::ai::{Ai, Math};
use crate::comp::avg::math::YearAround;

pub struct ScoreAi {
    pub plr: f32,
    year_value: f32,
}

pub const SCORE_AI: ScoreAi = ScoreAi {
    plr: 0.001,
    year_value: 5.2,
};

impl Ai {
    pub fn predict_match_score(data: &YearAround, team: &u16) -> f32 {
        if let Some(team_data) = Self::get_lock_find(team) {
            (Self::line_point_regression(&data.points.graph, Math::Score)
                + (Self::line_point_regression(&team_data.points.graph, Math::Score)
                    * SCORE_AI.year_value))
                / (1.0 + SCORE_AI.year_value)
        } else {
            Self::line_point_regression(&data.points.graph, Math::Score)
        }
    }
}
