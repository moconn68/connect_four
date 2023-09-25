use super::{ClientError, GameClient};
use crate::game::{
    board::GRID_COLS,
    state::{EndgameType, GameState},
};

use minimax::Strategy;

#[derive(Clone, Copy, Debug, Default)]
pub struct AiGameClient {
    game_state: GameState,
}

impl GameClient for AiGameClient {
    fn get_current_state(&self) -> GameState {
        self.game_state
    }
    fn handle_input_move(&mut self, column_num: usize) -> Result<GameState, ClientError> {
        let state_ref = &mut self.game_state;
        // First handle the user's move
        super::process_move(state_ref, column_num)?;
        // Now we generate and handle the AI's move
        let mut strategy = minimax::Negamax::new(NaiveEvaluator, 7);
        if let Some(ai_move) = strategy.choose_move(state_ref) {
            super::process_move(&mut self.game_state, ai_move)
            // Simulate thinking
            // TODO uncomment when UI threading behavior is reworked
            // std::thread::sleep(std::time::Duration::from_secs(1));
        } else {
            // Endgame, pass
            Ok(*state_ref)
        }
    }
}

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
