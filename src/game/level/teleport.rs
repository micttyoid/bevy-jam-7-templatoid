use bevy::prelude::*;
//use avian2d::prelude::*;
use crate::{
    screens::gameplay::GameplayLifetime,
    AppSystems, PausableSystems,
};


#[derive(Asset, Clone, Reflect)]
pub struct TelportAssets {
    pub aseprite: Handle<Aseprite>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}