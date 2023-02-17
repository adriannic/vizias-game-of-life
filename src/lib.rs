use std::{thread, time::Duration};

use vizia::prelude::*;

enum GameOfLifeEvent {
    ToggleGame,
    ToggleCell(usize, usize),
    Step,
}

#[derive(Lens)]
pub struct GameOfLife {
    running: bool,
    board: Vec<usize>,
    board_x: usize,
    board_y: usize,
}

impl GameOfLife {
    pub fn new(cx: &mut Context, board_x: usize, board_y: usize, delta_seconds: Duration) -> Handle<Self> {
        Self {
            running: false,
            board: vec![0; board_x * board_y],
            board_x,
            board_y,
        }
        .build(cx, move |cx| {
            cx.spawn(move |cx| loop {
                cx.emit(GameOfLifeEvent::Step).unwrap();
                thread::sleep(delta_seconds);
            })
            .build(cx);

            VStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    VStack::new(cx, move |cx| {
                        Binding::new(cx, GameOfLife::board, move |cx, board| {
                            for x in 0..board_x {
                                HStack::new(cx, move |cx| {
                                    for y in 0..board_y {
                                        let color = if board.get_val(cx)[y * board_x + x] == 1 {
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

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn step(&mut self) {
        if self.running {
            let mut new_board = self.board.clone();
            for (i, value) in self.board.iter().enumerate() {
                let neighbors = self.get_neighbors(i);
                let count: usize = neighbors.iter().map(|index| self.board[*index]).sum();
                new_board[i] =
                    if *value == 1 && (2..=3).contains(&count) || *value == 0 && count == 3 {
                        1
                    } else {
                        0
                    };
            }
            self.board = new_board;
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) -> bool {
        if self.running {
            return false;
        };
        self.board[y * self.board_x + x] = 1 - self.board[y * self.board_x + x];
        true
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) -> bool {
        if self.running {
            return false;
        };
        self.board[y * self.board_x + x] = value as usize;
        true
    }

    pub fn get_neighbors(&self, i: usize) -> Vec<usize> {
        let x = (i % self.board_x) as i32;
        let y = (i / self.board_x) as i32;

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
                let x = (x + dir_x).rem_euclid(self.board_x as i32) as usize;
                let y = (y + dir_y).rem_euclid(self.board_y as i32) as usize;

                y * self.board_x + x
            })
            .collect()
    }
}

impl View for GameOfLife {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|game_event, _| match game_event {
            GameOfLifeEvent::Step => self.step(),
            GameOfLifeEvent::ToggleGame => {
                if self.running {
                    self.stop()
                } else {
                    self.start()
                }
            }
            GameOfLifeEvent::ToggleCell(x, y) => {
                self.toggle_cell(*x, *y);
            }
        });
    }
}

#[cfg(test)]
mod tests {}
