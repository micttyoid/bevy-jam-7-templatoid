use bevy::prelude::*;

use avian2d::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    game::{
        animation::{PlayerAnimation, PlayerAnimationState, PlayerDirection},
        movement::{MovementController, ScreenWrap},
    },
};

pub const PLAYER_Z_TRANSLATION: f32 = 100.;
pub const PLAYER_COLLIDER_RADIUS: f32 = 12.;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.

    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// The player character.
pub fn player(max_speed: f32, player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        PlayerAnimation {
            state: PlayerAnimationState::default(),
            direction: PlayerDirection::default(),
        },
        AseAnimation {
            animation: Animation::tag("walk-up")
                .with_repeat(AnimationRepeat::Loop)
                .with_direction(AnimationDirection::Forward)
                .with_speed(2.0),
            aseprite: player_assets.player.clone(),
        },
        Sprite::default(),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        LockedAxes::new().lock_rotation(),
        Transform::from_xyz(0., 0., PLAYER_Z_TRANSLATION),
        // TODO: possibly kinematic later that should update `movement::apply_movement` along
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::circle(PLAYER_COLLIDER_RADIUS),
    )
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut MovementController,
            &mut PlayerAnimation,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    for (mut controller, mut animation, mut transform) in &mut query {
        let mut pressed_flag: bool = false;
        // Collect directional input.
        let mut intent = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) {
            intent.y += 1.0;
            animation.direction = PlayerDirection::Up;
            pressed_flag = true;
        }
        if input.pressed(KeyCode::KeyS) {
            intent.y -= 1.0;
            animation.direction = PlayerDirection::Down;
            pressed_flag = true;
        }
        if input.pressed(KeyCode::KeyA) {
            intent.x -= 1.0;
            animation.direction = PlayerDirection::Left;
            pressed_flag = true;
            transform.scale.x = -1.;
        }
        if input.pressed(KeyCode::KeyD) {
            intent.x += 1.0;
            animation.direction = PlayerDirection::Right;
            pressed_flag = true;
            transform.scale.x = 1.;
        }
        // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
        // This should be omitted if the input comes from an analog stick instead.
        let intent = intent.normalize_or_zero();
        controller.intent = intent;
        if pressed_flag {
            animation.state = PlayerAnimationState::Walk;
        } else {
            animation.state = PlayerAnimationState::Idle;
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    //#[dependancy]
    player: Handle<Aseprite>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("textures/chars/player.aseprite"),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
