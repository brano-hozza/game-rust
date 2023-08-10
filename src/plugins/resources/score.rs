use bevy::prelude::*;

use crate::plugins::events::GameOver;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score! Current score: {}", score.value)
    }
}

#[derive(Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores { scores: vec![] }
    }
}

pub fn update_high_scores(
    mut high_scores: ResMut<HighScores>,
    mut game_over_events: EventReader<GameOver>,
) {
    for event in game_over_events.iter() {
        high_scores
            .scores
            .push((String::from("Player"), event.score));

        println!("High scores:");
        println!("------------");
        for (name, score) in high_scores.scores.iter() {
            println!("{}: {}", name, score);
        }
        println!("------------");
    }
}
