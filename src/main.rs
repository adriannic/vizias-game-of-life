use vizia::prelude::*;
use vizia_test_view::GameOfLife;

fn main() {
    Application::new(|cx| {
        GameOfLife::new(cx);
    })
    .run();
}
