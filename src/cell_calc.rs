use bevy::prelude::*;

use crate::GRID;

pub struct CellCalcPlugin;

impl Plugin for CellCalcPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Board>();
    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone)]
pub struct Board(pub [i8; GRID*GRID]);

impl Default for Board {
    fn default() -> Self {
        Self([0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0])
    }
}

pub fn calculate_board(
    board: [i8; GRID*GRID],
) -> [i8; GRID*GRID] {
    let mut new_board = [0; GRID*GRID];

    for (i, state) in board.iter().enumerate() {
        let neighbours = calculate_neigbours(i, &board);

        if *state == 0 && neighbours == 3 {
            new_board[i] = 1;
        } else if *state == 1 && (neighbours < 2 || neighbours > 3) {
            new_board[i] = 0;
        } else {
            new_board[i] = *state;
        }
    }

    new_board
}

fn calculate_neigbours(i: usize, board: &[i8]) -> i8 {
    board[(i + board.len() - 1) % board.len()] +
    board[(i + board.len() + 1) % board.len()] +
    board[(i + board.len() - GRID) % board.len()] +
    board[(i + board.len() + GRID) % board.len()] +
    board[(i + board.len() - GRID+1) % board.len()] +
    board[(i + board.len() - GRID-1) % board.len()] +
    board[(i + board.len() + GRID-1) % board.len()] +
    board[(i + board.len() + GRID+1) % board.len()]
}