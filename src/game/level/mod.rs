pub mod enemies;
pub mod projectiles;

use avian2d::prelude::{Physics, PhysicsTime};

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::music,
    game::{
        player::{player},
        animation::AnimationAssets,
        level::{
            enemies::{basic_enemy, basic_boss,},
        },
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app
        .load_resource::<LevelAssets>()
        .init_state::<Level>()
        .add_plugins((
            enemies::plugin,
            projectiles::plugin,
        ));
}

/// GDD "pre defined multiple maps/levels(maybe 4-5?)"
/// TODO: Please name the levels according to the concept! ;o
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates, Reflect,)]
#[source(Screen = Screen::Gameplay)]
#[repr(u8)]
pub enum Level {
    #[default]
    Foo,
    Bar,
    Baz,
    Qux,
    Quux,
}

impl Level {
    pub const LAST_LEVEL: Level = Level::Quux;

    pub fn next(&self) -> Self {
        use Level::*;
        match self {
            Foo => Bar,
            Bar => Baz,
            Baz => Qux,
            Qux => Quux,
            Quux => panic!("No more next level: It is the last level"),
        }
    }

    pub fn is_last(&self) -> bool {
        if *self == Self::LAST_LEVEL {
            true
        } else {
            false
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    current_level: Res<State<Level>>,
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    anim_assets: Res<AnimationAssets>,
    mut time: ResMut<Time<Physics>>,
) {
    match current_level.get() {
        Level::Foo => {
            commands.spawn((
                Name::new("Level"),
                Transform::default(),
                Visibility::default(),
                DespawnOnExit(Screen::Gameplay),
                children![
                    player(100.0, &anim_assets),
                    basic_enemy((2., 5.).into(), &anim_assets),
                    basic_enemy((4., 5.).into(), &anim_assets),
                    basic_boss((6., 5.).into(), &anim_assets),
                    (
                        Name::new("Gameplay Music"),
                        music(level_assets.music.clone())
                    )
                ],
            ));
        }
        _ => {
            unimplemented!();
        }
    }
    time.unpause();
}