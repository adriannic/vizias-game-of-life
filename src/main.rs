use std::time::Duration;

use vizia::prelude::*;
use vizias_game_of_life::GameOfLife;

fn main() {
    Application::new(|cx| {
        GameOfLife::new(cx, 12, 10, Duration::from_millis(50));
    })
    .run();
}
