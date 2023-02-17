# Vizia's Game of Life

This is a custom view for Vizia to simulate Conway's Game of Life.

## Usage

When created, the view generates a grid with cells representing those of the
game and a button to start the game. I've decided to implement Life in a way
that allows cells on the borders to wrap around to the other side. While the
game is not running, the values of the cells can be toggled by clicking on them.
When the button is pressed, the game will start running. While the game is
running, the gamestate will change at regular intervals specified by
`delta_time` at the time of creation of the game. Cell values can't be changed
while the game is running. When the button is pressed again, the game will stop
running and it will be possible to change the values of the cells once again.
