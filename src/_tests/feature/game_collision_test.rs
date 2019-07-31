use crate::game::Game;
use crate::_tests::helpers::to_string_renderer::ToStringRenderer;
use crate::_tests::helpers::input_stub::{InputStub, InputStubRef};


struct TestableGame {
    input: InputStubRef,
    game: Game,
    renderer: ToStringRenderer,
}

impl TestableGame {
    pub fn with_field_height(field_height: u8) -> Self {
        let input = InputStub::new_rc();
        Self {
            input: input.clone(),
            game: Game::init()
                .with_input(input.clone())
                .with_field_height(field_height).build(),
            renderer: ToStringRenderer::with_height(field_height as usize),
        }
    }

    pub fn is_moving_left(&mut self) {
        self.input.borrow_mut().is_moving_left();
    }

    pub fn is_moving_right(&mut self) {
        self.input.borrow_mut().is_moving_right();
    }

    pub fn is_not_moving(&mut self) {
        self.input.borrow_mut().is_not_moving();
    }

    pub fn tick(&mut self, now: u64) {
        self.game.tick(now);
    }

    pub fn verify_frame_after(&mut self, now: u64, expected_frame: Vec<&str>) {
        self.game.tick(now);
        self.game.render(&mut self.renderer);
        self.renderer.assert_frame_exact(expected_frame);
    }
}

#[test]
fn when_hitting_ground_spawn_another_brick_after_100_ms() {
    let mut game = TestableGame::with_field_height(4);

    game.is_moving_right();
    game.tick(100);
    game.tick(200);
    game.verify_frame_after(300, vec![
        "..........",
        "..........",
        "..........",
        "....####.."
    ]);

    game.is_not_moving();
    game.verify_frame_after(400, vec![
        ".####.....",
        "..........",
        "..........",
        "....####.."
    ]);

    game.is_moving_left();
    game.verify_frame_after(500, vec![
        "..........",
        "####......",
        "..........",
        "....####.."
    ]);
}

#[test]
fn when_hitting_another_brick_brick_stops_right_there() {
    let mut game = TestableGame::with_field_height(4);

    game.is_moving_right();
    game.tick(100);
    game.tick(200);
    game.verify_frame_after(300, vec![
        "..........",
        "..........",
        "..........",
        "....####.."
    ]);

    game.is_not_moving();
    game.tick(400);
    game.tick(500);
    game.verify_frame_after(600, vec![
        "..........",
        "..........",
        ".####.....",
        "....####.."
    ]);

    game.verify_frame_after(700, vec![
        ".####.....",
        "..........",
        ".####.....",
        "....####.."
    ]);
}
