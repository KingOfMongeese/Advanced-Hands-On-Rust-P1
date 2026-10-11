use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod components;
pub mod resources;

mod systems;
pub use systems::*;

#[derive(Debug, Clone, Copy, PartialEq, States, Eq, Hash, Default)]
enum GamePhase {
    #[default]
    Player,

    Cpu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .add_systems(Update, display_score)
        .add_systems(Update, player.run_if(in_state(GamePhase::Player)))
        .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
        .run();
}
