use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct Icons {
    pub chevron_down: Handle<Image>,
    pub chevron_up: Handle<Image>,
}

pub fn setup_icons(asset_server: Res<AssetServer>, mut icons: ResMut<Icons>) {
    icons.chevron_down = asset_server.load("icons/chevron-down-solid.png");
    icons.chevron_up = asset_server.load("icons/chevron-up-solid.png");
}
