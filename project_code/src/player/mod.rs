use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::components::GameState;
use crate::GameworldState;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
         app.add_systems(OnEnter(GameworldState::Island), (
                initial_spawn_player,
                apply_deferred,
                spawn_weapon.after(initial_spawn_player),
                ))  
            .add_systems(OnEnter(GameworldState::Dungeon), (
                initial_spawn_player,
                apply_deferred,
                spawn_weapon.after(initial_spawn_player),))
            .add_systems(Update, (    
                move_player,
                player_animation.after(move_player),
                sword_attack,
                sword_swoosh_animation,
                musket_attack,
                dagger_attack,
                pistol_attack,
                check_player_health,
                musketball_lifetime_check,
                move_musketball,
                move_weapon.after(move_player),
                swap_weapon,
            )
                .run_if(in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)))
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            OnExit(GameworldState::Island),
            (despawn_player, despawn_musketballs),
        )
        .add_systems(
            OnExit(GameworldState::Dungeon),
            (despawn_player, despawn_musketballs),
        );
    }
}
