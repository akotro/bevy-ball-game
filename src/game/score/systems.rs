use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOverEvent;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        if high_scores.scores.is_empty() || event.final_score > high_scores.scores.last().unwrap().1
        {
            high_scores
                .scores
                .push(("Player".to_string(), event.final_score));
        }
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
