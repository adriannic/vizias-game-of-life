use std::time::Duration;

use vizia::prelude::*;
use vizias_game_of_life::GameOfLife;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("./styles.css").unwrap();
        GameOfLife::new(cx, 25, 25, Duration::from_millis(50));
    })
    .run();
}
