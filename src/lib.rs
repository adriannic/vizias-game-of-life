//! # Vizia's Game of Life
//!
//! This is a custom view for Vizia to simulate Conway's Game of Life.
//!
//! ## Usage
//! When created, the view generates a grid with cells representing those of the game and a button
//! to start the game. I've decided to implement Life in a way that allows cells on the borders to
//! wrap around to the other side. While the game is not running, the values of the cells can be
//! toggled by clicking on them. When the button is pressed, the game will start running. While the
//! game is running, the gamestate will change at regular intervals specified by `delta_time` at
//! the time of creation of the game. Cell values can't be changed while the game is running. When
//! the button is pressed again, the game will stop running and it will be possible to change the
//! values of the cells once again.

use std::{thread, time::Duration};

use vizia::prelude::*;

/// Events used by Vizia to change the state of the game.
enum GameOfLifeEvent {
    ToggleGame,
    ToggleCell(usize, usize),
    Step,
}

impl View for GameOfLife {
    /// Handles events related to the game.
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|game_event, meta| match game_event {
            GameOfLifeEvent::Step => {
                self.step();
                meta.consume();
            }
            GameOfLifeEvent::ToggleGame => {
                if self.running {
                    self.stop()
                } else {
                    self.start()
                }
                meta.consume();
            }
            GameOfLifeEvent::ToggleCell(x, y) => {
                self.toggle_cell(*x, *y);
                meta.consume();
            }
        });
    }
}

/// Conway's game of life implemented as a Vizia view
#[derive(Lens)]
pub struct GameOfLife {
    running: bool,
    board: Vec<usize>,
    board_x: usize,
    board_y: usize,
}

impl GameOfLife {
    /// Creates a new Game of Life instance.
    ///
    /// # Arguments
    /// * `cx` - The context where the view is set in.
    /// * `board_x` - Horizontal size of the board.
    /// * `board_y` - Vertical size of the board.
    /// * `delta_time` - Time interval between game ticks.
    pub fn new(
        cx: &mut Context,
        board_x: usize,
        board_y: usize,
        delta_time: Duration,
    ) -> Handle<Self> {
        Self {
            running: false,
            board: vec![0; board_x * board_y],
            board_x,
            board_y,
        }
        .build(cx, move |cx| {
            cx.spawn(move |cx| loop {
                cx.emit(GameOfLifeEvent::Step).unwrap();
                thread::sleep(delta_time);
            })
            .build(cx);

            VStack::new(cx, move |cx| {
                Label::new(cx, "Vizia's Game of Life")
                    .class("title");

                for x in 0..board_x {
                    HStack::new(cx, move |cx| {
                        for y in 0..board_y {
                            Element::new(cx)
                                .class("tile")
                                .checked(GameOfLife::board.map(move |v| v[y * board_x + x] == 1))
                                .on_press(move |cx| cx.emit(GameOfLifeEvent::ToggleCell(x, y)));
                        }
                    })
                    .class("tile-row");
                }

                Button::new(
                    cx,
                    |cx| cx.emit(GameOfLifeEvent::ToggleGame),
                    |cx| {
                        Label::new(
                            cx,
                            GameOfLife::running.map(|v| if *v { "Stop" } else { "Start" }),
                        )
                        .color(Color::white())
                    },
                )
                .class("action-button");
            })
            .class("tiles-wrapper");
        })
    }

    /// Starts the game.
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stops the game.
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Generates the next gamestate.
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

    /// Toggles the state of the cell at pos `(x, y)`.
    /// Returns `true` if the state could be set, `false` otherwise.
    ///
    /// # Arguments
    /// * `x` - X coordinate of the cell.
    /// * `y` - Y coordinate of the cell.
    pub fn toggle_cell(&mut self, x: usize, y: usize) -> bool {
        if self.running {
            return false;
        };
        self.board[y * self.board_x + x] = 1 - self.board[y * self.board_x + x];
        true
    }

    /// Set the state of the cell at pos `(x, y)` to `value`.
    /// Returns `true` if the state could be set, `false` otherwise.
    ///
    /// # Arguments
    /// * `x` - X coordinate of the cell.
    /// * `y` - Y coordinate of the cell.
    /// * `value` - Value to set the cell to.
    pub fn set_cell(&mut self, x: usize, y: usize, value: bool) -> bool {
        if self.running {
            return false;
        };
        self.board[y * self.board_x + x] = value as usize;
        true
    }

    /// Returns the cells neighboring the cell with index `i`.
    ///
    /// # Arguments
    /// * `i` - Index of the cell.
    fn get_neighbors(&self, i: usize) -> Vec<usize> {
        let x = (i % self.board_x) as i32;
        let y = (i / self.board_x) as i32;

        // Directions (all 8)
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(dir_x, dir_y)| {
            let x = (x + dir_x).rem_euclid(self.board_x as i32) as usize;
            let y = (y + dir_y).rem_euclid(self.board_y as i32) as usize;

            y * self.board_x + x
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {}
