mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use self::systems::{interactions::*, layout::*};
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // NOTE: OnEnter state systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // NOTE: Systems
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // NOTE: OnExit state systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
