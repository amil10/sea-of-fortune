use crate::{components::BoundingBox, shop::components::Inventory};
use crate::network::components::HostPlayer;
use bevy::prelude::*;

/// The speed at which the player accelerates
pub const PLAYER_ACCELERATION: f32 = 5000.;
pub const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: f32 = 32.;
pub const PLAYER_ANIMATION_TIME: f32 = 0.1;

/// Musketball info
pub const MUSKETBALL_SPEED: f32 = 500.;
pub const MUSKETBALL_LIFETIME: f32 = 6.;
pub const MAX_ACCEL: f32 = 800.;

pub const SWORD_COOLDOWN: f32 = 0.75;
pub const MUSKET_COOLDOWN: f32 = 1.5;
pub const DAGGER_COOLDOWN: f32 = 0.375; // Half of sword cooldown
pub const PISTOL_COOLDOWN: f32 = 0.75;  // Half of musket cooldown

// Base player stats
pub const PLAYER_MAX_HP: f32 = 3.;

#[derive(Component)]
pub struct PlayerVisibility {
    pub visible: bool
}


#[derive(Component)]
pub struct DespawnWeapon {
    pub should_despawn: bool,
}

impl Default for DespawnWeapon {
    fn default() -> Self {
        Self {
            should_despawn: false,
        }
    }
}

/// Struct representing the player
#[derive(Component)]
pub struct Player {
    pub host_id: i32,
    pub animation_state: SpriteState,
    pub timer: Timer,
    pub health: f32,
    pub max_health: f32,
    pub inventory: Inventory,
    pub spawn_position: Vec3,
    pub weapon: i8,
    pub aabb: BoundingBox,
}

/// Struct representing the musketball projectile fired by the musket weapon
#[derive(Component)]
pub struct Musketball;

/// Struct representation for musketball projectile lifetime
#[derive(Component)]
pub struct MusketballLifetime(pub f32);

/// Musketball velocity struct
#[derive(Component)]
pub struct MusketballVelocity {
    pub v: Vec3,
}

#[derive(Component)]
pub struct SwordSwooshAnimation {
    pub timer: Timer,
    pub active: bool,
}


/// Struct representing the sword weapon for the player
#[derive(Component)]
pub struct Sword {
    pub damage: f32,
    pub upgraded: bool,
}

//implementing sword
impl Default for Sword {
    /// Sets default values for the sword
    fn default() -> Sword {
        Sword {
            damage: 1.,      //unupgraded damage
            upgraded: false, //upgraded damage
        }
    }
}

// Sword Damage and Upgrade Mechanics
impl Sword {
    pub fn get_base_damage(&self) -> f32 {
        if self.upgraded {
            2.0 // Base damage for upgraded sword
        } else {
            1.0 // Base damage for basic sword
        }
    }

    pub fn get_level_multiplier(level: u32) -> f32 {
        // Each level adds 25% more damage
        1.0 + (level as f32 * 0.25)
    }

    pub fn calculate_damage(&self, level: u32) -> f32 {
        self.get_base_damage() * Self::get_level_multiplier(level)
    }

    pub fn upgrade_sword(&mut self, level: u32) {
        self.damage = self.calculate_damage(level);
        self.upgraded = true;
    }
}

/// Struct representing the dagger weapon for the player
#[derive(Component)]
pub struct Dagger {
    pub damage: f32,
    pub upgraded: bool,
}

//implementing dagger
impl Default for Dagger {
    fn default() -> Dagger {
        Dagger {
            damage: 0.5,     // Half of sword's base damage
            upgraded: false,
        }
    }
}

// Dagger Damage and Upgrade Mechanics
impl Dagger {
    pub fn get_base_damage(&self) -> f32 {
        if self.upgraded {
            1.0 // Base damage for upgraded dagger (half of sword)
        } else {
            0.5 // Base damage for basic dagger
        }
    }

    pub fn get_level_multiplier(level: u32) -> f32 {
        // Each level adds 25% more damage (same as sword)
        1.0 + (level as f32 * 0.25)
    }

    pub fn calculate_damage(&self, level: u32) -> f32 {
        self.get_base_damage() * Self::get_level_multiplier(level)
    }

    pub fn upgrade_dagger(&mut self, level: u32) {
        self.damage = self.calculate_damage(level);
        self.upgraded = true;
    }
}

/// Struct representing the musket weapon for the player
#[derive(Component)]
pub struct Musket {
    pub damage: f32,
    pub upgraded: bool,
}

impl Default for Musket {
    fn default() -> Musket {
        Musket {
            damage: 1.0,     // Base musket damage
            upgraded: false,
        }
    }
}

// Musket Damage and Upgrade Mechanics
impl Musket {
    pub fn get_base_damage(&self) -> f32 {
        if self.upgraded {
            2.0 // Base damage for upgraded musket
        } else {
            1.0 // Base damage for basic musket
        }
    }

    pub fn get_level_multiplier(level: u32) -> f32 {
        // Each level adds 25% more damage
        1.0 + (level as f32 * 0.25)
    }

    pub fn calculate_damage(&self, level: u32) -> f32 {
        self.get_base_damage() * Self::get_level_multiplier(level)
    }

    pub fn upgrade_musket(&mut self, level: u32) {
        self.damage = self.calculate_damage(level);
        self.upgraded = true;
    }
}

/// Struct representing the pistol weapon for the player
#[derive(Component)]
pub struct Pistol {
    pub damage: f32,
    pub upgraded: bool,
}

//implementing pistol
impl Default for Pistol {
    fn default() -> Pistol {
        Pistol {
            damage: 0.5,     // Half of musket's base damage
            upgraded: false,
        }
    }
}

// Pistol Damage and Upgrade Mechanics
impl Pistol {
    pub fn get_base_damage(&self) -> f32 {
        if self.upgraded {
            1.0 // Base damage for upgraded pistol (half of musket)
        } else {
            0.5 // Base damage for basic pistol
        }
    }

    pub fn get_level_multiplier(level: u32) -> f32 {
        // Each level adds 25% more damage (same as musket)
        1.0 + (level as f32 * 0.25)
    }

    pub fn calculate_damage(&self, level: u32) -> f32 {
        self.get_base_damage() * Self::get_level_multiplier(level)
    }

    pub fn upgrade_pistol(&mut self, level: u32) {
        self.damage = self.calculate_damage(level);
        self.upgraded = true;
    }
}

/// Velocity struct
#[derive(Component)]
pub struct Velocity {
    pub v: Vec2,
}

/// Struct for tracking players last position for out of transition spawning
pub struct PlayerLastPosition {
    pub last_pos: Vec2,
}

/// Velocity implementation
impl Velocity {
    pub fn new() -> Self {
        Self {
            //sets x and y velocity to 0
            v: Vec2::splat(0.),
        }
    }

    pub fn to_vec3(&self, z: f32) -> Vec3 {
        Vec3::new(self.v.x, self.v.y, z)
    }
}

/// Struct for the time between frames of animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    /// Initializes the animation timer
    pub fn new(timer: Timer) -> AnimationTimer {
        AnimationTimer(timer)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct TestTimer(Timer);

impl TestTimer {
    pub fn new(timer: Timer) -> TestTimer {
        TestTimer(timer)
    }
}

/// Struct for the count of frames in the players animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

impl AnimationFrameCount {
    /// Initializes the animation frame count
    pub fn new(size: usize) -> AnimationFrameCount {
        AnimationFrameCount(size)
    }
}

/// Struct that represents the current sprite for the players state
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpriteState {
    Idle,
    LeftRun,
    RightRun,
    BackwardRun,
    ForwardRun,
}

impl SpriteState {
    /// Matches the player animation to the current player
    /// state
    pub fn animation_indices(&self) -> std::ops::Range<usize> {
        match self {
            SpriteState::Idle => 0..8,
            SpriteState::LeftRun => 8..16,
            SpriteState::RightRun => 16..24,
            SpriteState::ForwardRun => 24..32,
            SpriteState::BackwardRun => 32..40,
        }
    }

    /// Matches the speed of animation to the animation being
    /// played
    pub fn animation_speed(&self) -> f32 {
        match self {
            SpriteState::Idle => 0.1,
            SpriteState::LeftRun => 0.1,
            SpriteState::RightRun => 0.1,
            SpriteState::BackwardRun => 0.1,
            SpriteState::ForwardRun => 0.1,
        }
    }
}

///Struct that keeps track of the cooldown between attacks
#[derive(Component)]
pub struct AttackCooldown {
    pub remaining: Timer,
}
