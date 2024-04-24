use bevy::prelude::*;

use crate::GRID;

pub struct CellCalcPlugin;

impl Plugin for CellCalcPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CellState>();
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub struct CellState([i8; GRID*GRID]);

pub fn calculate_board(
    board: Res<State<CellState>>,
    mut next_board: ResMut<NextState<CellState>>,
) {
    let mut new_board = [0; GRID*GRID];

    for (i, state) in board.get().0.iter().enumerate() {
        let neighbours = calculate_neigbours(i, &board.get().0);

        if *state == 0 && neighbours == 3 {
            new_board[i] = 1;
        } else if *state == 1 && (neighbours < 2 || neighbours > 3) {
            new_board[i] = 0;
        } else {
            new_board[i] = *state;
        }
    }
    println!("{:?}", new_board);

    next_board.set(CellState(new_board))
}

fn calculate_neigbours(i: usize, board: &[i8]) -> i8 {
    board[(i + GRID*GRID - 1) % GRID*GRID] +
    board[(i + GRID*GRID + 1) % GRID*GRID] +
    board[(i + GRID*GRID - 4) % GRID*GRID] +
    board[(i + GRID*GRID + 4) % GRID*GRID] +
    board[(i + GRID*GRID - 5) % GRID*GRID] +
    board[(i + GRID*GRID - 3) % GRID*GRID] +
    board[(i + GRID*GRID + 3) % GRID*GRID] +
    board[(i + GRID*GRID + 5) % GRID*GRID]
}