use vizia::prelude::*;
use vizias_game_of_life::GameOfLife;

fn main() {
    Application::new(|cx| {
        GameOfLife::new(cx);
    })
    .run();
}
