use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use macroquad_particles::*;

fn emitter_config() -> EmitterConfig {
    EmitterConfig {
        lifetime: 5.0,
        lifetime_randomness: 0.1,
        amount: 10,
        emitting: false,
        initial_direction_spread: 0.1,
        initial_velocity: 16.0,
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        initial_size: 0.75,
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

#[macroquad::main("Particle Overflow Repro")]
async fn main() {

    let texture = load_texture("raw/exhaust.png").await;

    let mut emitter = Emitter::new(EmitterConfig {
        texture: Some(texture),
        local_coords: false,
        ..emitter_config()
    });

    loop {

        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;

        // this seems to be the pertinent bit, lifetime was sometimes 0 initially?
        emitter.config.lifetime = 0.0;

        emitter.emit(vec2(x, y), 16);
        emitter.draw(vec2(0.0, 0.0));

        next_frame().await;

    }

}