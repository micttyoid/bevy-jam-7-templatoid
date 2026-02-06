use bevy::prelude::*;
pub mod tiled;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        tiled::plugin,
    ));
}
