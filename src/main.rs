mod rust_invaders;
mod audio;
mod systems;

use crate::rust_invaders::*;
use amethyst::prelude::*;
use amethyst::{
    renderer::{
        plugins::{RenderFlat3D,RenderBase3D,RenderPbr3D,  RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::{application_dir, application_root_dir},
    core::transform::TransformBundle,
    input::{InputBundle, Bindings},
    ui::RenderUi
};
use amethyst::input::StringBindings;


fn main() -> amethyst::Result<()> {
    // Begin logger
    amethyst::start_logger(Default::default());
    // Setup assets directory, and config files
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    // Setup Input bindings
    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;


    let game_data = GameDataBuilder::default().with_bundle(RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.10196,0.1,0.11765, 1.0])
        )
        .with_plugin(RenderPbr3D::default())
    )?;

    let mut game = Application::new(assets_dir,rust_invaders::RustInvaders::default(), game_data)?;

    game.run();

    Ok(())
}
