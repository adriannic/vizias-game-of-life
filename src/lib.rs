use vizia::prelude::*;

const TEST_VIEW_SIZE: usize = 10;

enum GameOfLifeEvent {
    ToggleGame,
}

#[derive(Clone, PartialEq, Eq)]
pub enum GameOfLifeState {
    Running,
    Stopped,
}

impl Data for GameOfLifeState {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

impl std::fmt::Display for GameOfLifeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Stopped => "Begin",
            Self::Running => "Stop",
        };
        write!(f, "{}", string)
    }
}

#[derive(Lens)]
pub struct GameOfLife {
    state: GameOfLifeState,
}

impl GameOfLife {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            state: GameOfLifeState::Stopped,
        }
        .build(cx, |cx| {
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    for _ in 0..TEST_VIEW_SIZE {
                        Label::new(cx, "█ █ █ █ █ █ █ █ █ █")
                            .background_color(Color::rgb(18, 18, 18));
                    }
                })
                .border_width(Pixels(5.0))
                .border_color(Color::rgb(127, 127, 127))
                .child_space(Stretch(1.0))
                .border_radius(Pixels(10.0));

                Button::new(
                    cx,
                    |cx| cx.emit(GameOfLifeEvent::ToggleGame),
                    |cx| Label::new(cx, GameOfLife::state).color(Color::white()),
                )
                .background_color(Color::rgb(18, 18, 18))
                .top(Pixels(10.0));
            })
            .background_color(Color::black())
            .color(Color::white())
            .child_space(Stretch(1.0));
        })
    }
}

impl View for GameOfLife {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|game_event, _| match game_event {
            GameOfLifeEvent::ToggleGame => {
                self.state = match self.state {
                    GameOfLifeState::Running => GameOfLifeState::Stopped,
                    GameOfLifeState::Stopped => GameOfLifeState::Running,
                };
            }
        });
    }
}

#[cfg(test)]
mod tests {}
