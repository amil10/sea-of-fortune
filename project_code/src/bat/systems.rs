use bevy::prelude::*;

use crate::bat::components::*;
use crate::player::components::Player;
use crate::data::gameworld_data::*;

/*   ROTATE_BAT FUNCTION   */
/// This should be changed to a function called "track_player", which will
/// be how the bat knows where to check where the player is for shooting projectiles
/// 
/// WE DON'T NEED TO ROTATE THE BAT! I WILL MAKE A BACK FACING SPRITE IF NEEDED
pub fn rotate_bat(
    time: Res<Time>,
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // getting player position
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (config, mut enemy_transform) in &mut query {
        //getting bat's position relative to player position
        let bat_position = enemy_transform.translation.xy();
        let distance_to_player = bat_position.distance(player_translation);

        //ensuring bat is close enough to player to attack
        if distance_to_player > ATTACK_DIST {
            break;
        }


        //getting enemy forward
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        let forward_dot_player = enemy_forward.dot(to_player);


        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }


        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        let right_dot_player = enemy_right.dot(to_player);

        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos(); 

        let rotation_angle =
            rotation_sign * (config.rotation_speed * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }
}

/*   ANIMATE_BAT FUNCTION   */
/// Animates a bat entity
pub fn animate_bat(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &AnimationFrameCount)>,
) {
    for (mut timer, mut texture_atlas, frame_count) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
    }
}

/*  SPAWN_BAT FUNCTION  */
/// Spawns a bat entity in the gameworld
pub fn spawn_bat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //getting bat sprite information
    let bat_sheet_handle = asset_server.load("s_bat.png");
    let bat_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);
    let bat_layout_len = 3;
    let bat_layout_handle = texture_atlases.add(bat_layout.clone());

    //spawning bat and setting bat information
    commands.spawn((
        SpriteBundle {
            texture: bat_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        Bat {
            rotation_speed: f32::to_radians(90.0),
        },
        RotateToPlayer {
            rotation_speed: f32::to_radians(90.0),
        },
        TextureAtlas {
            layout: bat_layout_handle,
            index: 0,
        },
        AnimationTimer::new(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating)),
        AnimationFrameCount::new(bat_layout_len),
        Velocity::new(),
    ));
}