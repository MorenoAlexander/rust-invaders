mod rust_invaders;
mod audio;
mod systems;

use crate::rust_invaders::*;
use amethyst::prelude::*;
use amethyst::{
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::{application_dir, application_root_dir},
    core::transform::TransformBundle,
    input::{InputBundle, Bindings},
};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    // TODO SET UP CONFIG
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default().with_bundle(RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.00196,0.2376,0.21765, 1.0])
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
    );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir,rust_invaders::default(), game_data);

    Ok(())
}
