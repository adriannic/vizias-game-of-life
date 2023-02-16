use std::{thread, time::Duration};

use vizia::prelude::*;

const TEST_VIEW_SIZE: usize = 10;

enum GameOfLifeEvent {
    ToggleGame,
    ToggleCell(usize, usize),
    Step,
}

#[derive(Lens)]
pub struct GameOfLife {
    running: bool,
    board: Vec<usize>,
}

impl GameOfLife {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            running: false,
            board: vec![0; TEST_VIEW_SIZE * TEST_VIEW_SIZE],
        }
        .build(cx, |cx| {
            cx.spawn(|cx| loop {
                cx.emit(GameOfLifeEvent::Step).unwrap();
                thread::sleep(Duration::from_secs(1));
            })
            .build(cx);

            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Binding::new(cx, GameOfLife::board, |cx, board| {
                            for x in 0..TEST_VIEW_SIZE {
                                HStack::new(cx, |cx| {
                                    for y in 0..TEST_VIEW_SIZE {
                                        let color =
                                            if board.get_val(cx)[y * TEST_VIEW_SIZE + x] == 1 {
                                                Color::white()
                                            } else {
                                                Color::rgb(96, 96, 96)
                                            };

                                        Label::new(cx, "     ")
                                            .background_color(Color::rgb(96, 96, 96))
                                            .left(Pixels(1.5))
                                            .right(Pixels(1.5))
                                            .bottom(Pixels(1.5))
                                            .top(Pixels(1.5))
                                            .border_radius(Pixels(2.0))
                                            .on_press(move |cx| {
                                                cx.emit(GameOfLifeEvent::ToggleCell(x, y))
                                            })
                                            .background_color(color);
                                    }
                                });
                            }
                        })
                    })
                    .child_space(Stretch(1.0))
                    .border_radius(Pixels(5.0));

                    Binding::new(cx, GameOfLife::running, |cx, running| {
                        let button_text = if running.get_val(cx) { "Stop" } else { "Start" };
                        Button::new(
                            cx,
                            |cx| cx.emit(GameOfLifeEvent::ToggleGame),
                            |cx| Label::new(cx, button_text).color(Color::white()),
                        )
                        .background_color(Color::rgb(127, 127, 127))
                        .top(Pixels(10.0));
                    });
                })
                .background_color(Color::rgb(32, 32, 32))
                .color(Color::white())
                .child_space(Stretch(1.0))
                .border_width(Pixels(5.0))
                .border_radius(Pixels(10.0))
                .space(Stretch(0.5));
            })
            .background_color(Color::rgb(18, 18, 18))
            .color(Color::white())
            .child_space(Stretch(1.0))
            .border_width(Pixels(5.0))
            .border_radius(Pixels(10.0))
            .space(Auto);
        })
    }
}

impl View for GameOfLife {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|game_event, _| match game_event {
            GameOfLifeEvent::Step => {
                if self.running {
                    let mut new_board = self.board.clone();
                    for (i, value) in self.board.iter().enumerate() {
                        let neighbors = get_neighbors(i, TEST_VIEW_SIZE);
                        let count: usize = neighbors.iter().map(|index| self.board[*index]).sum();
                        new_board[i] = if *value == 1 && (2..=3).contains(&count)
                            || *value == 0 && count == 3
                        {
                            1
                        } else {
                            0
                        };
                    }
                    self.board = new_board;
                }
            }
            GameOfLifeEvent::ToggleGame => self.running ^= true,
            GameOfLifeEvent::ToggleCell(x, y) => {
                if !self.running {
                    self.board[y * TEST_VIEW_SIZE + x] = 1 - self.board[y * TEST_VIEW_SIZE + x];
                }
            }
        });
    }
}

fn get_neighbors(i: usize, row_len: usize) -> Vec<usize> {
    let x = (i % row_len) as i32;
    let y = (i / row_len) as i32;

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    DIRECTIONS
        .iter()
        .map(|(dir_x, dir_y)| {
            let x = (x + dir_x).rem_euclid(row_len as i32) as usize;
            let y = (y + dir_y).rem_euclid(row_len as i32) as usize;

            y * row_len + x
        })
        .collect()
}

#[cfg(test)]
mod tests {}
