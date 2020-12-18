use sfml::{
    window::{Style, VideoMode, Event, Key}, 
    graphics::{
        RenderWindow,
        Transformable,
        Color,
        RenderTarget,
        Texture,
        Sprite,
        Drawable, Rect,
    }, 
    system::{Clock, Vector2f}
};

#[derive(Debug)]
struct Player<'a> {
    position: Vector2f,
    velocity: Vector2f,
    control_left: bool,
    control_right: bool,
    control_up: bool,
    control_down: bool,
    control_jump: bool,
    sprite: Sprite<'a>,
}
impl<'a> Player<'a> {
    fn new(x: f32, y: f32, texture: &'a Texture) -> Self {
        let mut sprite = Sprite::with_texture(texture);
        sprite.set_position((x, y));
        Self {
            position: Vector2f::new(x, y),
            velocity: Vector2f::new(0., 0.),
            control_left: false,
            control_right: false,
            control_up: false,
            control_down: false,
            control_jump: false,
            sprite,
        }
    }

    fn move_player(&mut self, _delta_time_ms: i32) {
        let x = self.sprite.position().x;
        let y = self.sprite.position().y;
        if self.control_left {
            &self.sprite.set_position((x - 1., y));
        }
        if self.control_right {
            &self.sprite.set_position((x + 1., y));

        }
        if self.control_down {
            &self.sprite.set_position((x, y + 1.));

        }
        if self.control_up {
            &self.sprite.set_position((x, y - 1.));
        }
    }
}
impl Drawable for Player<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn sfml::graphics::RenderTarget,
        render_states: sfml::graphics::RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        &self.sprite.draw(target, render_states);
    }
}

fn main() {
    let window_height = 600;
    let window_width = 800;
    let fps = 60;
    let _frame_time: f32 = 1./(fps as f32);
    let frame_time_ms = 1/fps * 1000;

    let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
    VideoMode::new(window_width, window_height, desktop.bits_per_pixel),
    "Rusty Pig Platformer",
    Style::CLOSE,
    &Default::default()
    );
    window.set_vertical_sync_enabled(true);
    window.set_framerate_limit(60);

    let pig_img = include_bytes!("../resources/models/pig.png");
    let pig_texture = Texture::from_memory(pig_img, &Rect::new(0, 0, 64, 64))
        .unwrap();
    let mut player = Player::new(100., 100., &pig_texture);

    let mut clock = Clock::start();
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code, .. } => match code {
                    Key::A => player.control_left = true,
                    Key::D => player.control_right = true,
                    Key::W => player.control_up = true,
                    Key::S => player.control_down = true,
                    Key::Space => player.control_jump = true,
                    Key::Escape => return,
                    _ => (),
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::A => player.control_left = false,
                    Key::D => player.control_right = false,
                    Key::W => player.control_up = false,
                    Key::S => player.control_down = false,
                    Key::Space => player.control_jump = false,
                    _ => (),
                },
                _ => (),
            }
        }
        if frame_time_ms < clock.elapsed_time().as_milliseconds() {
            let delta_time = clock.elapsed_time().as_milliseconds();
            player.move_player(delta_time);
            window.clear(Color::BLACK);
            window.draw(&player);
            window.display();
            clock.restart();
        }
    }
}
