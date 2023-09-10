use crate::game::{
    board::GRID_COLS,
    state::{EndgameType, GameState},
};

pub struct FourStackRules;
impl minimax::Game for FourStackRules {
    type S = GameState;
    type M = usize;

    fn generate_moves(state: &Self::S, moves: &mut Vec<Self::M>) {
        for i in 1..GRID_COLS {
            let mut board = *state.get_board();
            if board.insert_piece(*state.get_next_player(), i).is_ok() {
                moves.push(i);
            }
        }
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        let player = *state.get_next_player();
        let board = state.get_board_mut();
        board.insert_piece(player, m).ok()?;
        Some(*state)
    }

    fn get_winner(state: &Self::S) -> Option<minimax::Winner> {
        use minimax::Winner;
        match state.check_endgame() {
            EndgameType::Full => Some(Winner::Draw),
            EndgameType::None => None,
            EndgameType::Win(p) => Some({
                if p.eq(state.get_next_player()) {
                    Winner::PlayerJustMoved
                } else {
                    Winner::PlayerToMove
                }
            }),
        }
    }
}

// TODO need to improve this
pub struct NaiveEvaluator;
impl minimax::Evaluator for NaiveEvaluator {
    type G = FourStackRules;

    fn evaluate(&self, s: &<Self::G as minimax::Game>::S) -> minimax::Evaluation {
        match s.check_endgame() {
            EndgameType::Win(player) => match player.eq(s.get_next_player()) {
                true => -1,
                false => 1,
            },
            _ => 0,
        }
    }
}
