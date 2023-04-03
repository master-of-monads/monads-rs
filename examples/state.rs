#![cfg(feature = "state")]

use std::fmt;

use monads_rs::state::State;
use monads_rs::*;

fn main() {
	let state = program().run(GameState::new());
	println!("{}", state.0);
}

type GameStateState<A> = State<'static, GameState, A>;

#[monadic]
fn program() -> GameStateState<()> {
	play_turn(0, 0)?;
	play_turn(0, 2)?;
	play_turn(2, 0)?;
	play_turn(1, 0)?;
	play_turn(2, 2)?;
	play_turn(2, 1)?;
	play_turn(1, 1)
}

#[monadic]
fn play_turn(x: usize, y: usize) -> GameStateState<()> {
	set_piece(x, y)?;
	switch_current_player()?;
	inc_round_counter()
}

#[monadic]
fn set_piece(x: usize, y: usize) -> GameStateState<()> {
	let board = get_board()?;
	let player = get_current_player()?;
	set_board(board_set_piece(board, x, y, player))
}

// Getters
#[monadic]
fn get_board() -> GameStateState<Board> {
	let state = GameStateState::<GameState>::get()?;
	State::ret(state.board)
}

#[monadic]
fn get_current_player() -> GameStateState<Player> {
	let state = GameStateState::<GameState>::get()?;
	State::ret(state.turn)
}

// Setters
#[monadic]
fn set_board(board: Board) -> GameStateState<()> {
	let mut state = GameStateState::<GameState>::get()?;
	state.board = board;
	GameStateState::<()>::set(state)
}

#[monadic]
fn switch_current_player() -> GameStateState<()> {
	let mut state = GameStateState::<GameState>::get()?;
	state.turn = state.turn.flip();
	GameStateState::<()>::set(state)
}

#[monadic]
fn inc_round_counter() -> GameStateState<()> {
	let mut state = GameStateState::<GameState>::get()?;
	state.round_count += 1;
	return GameStateState::<()>::set(state);
}

#[derive(Clone, Copy, Debug)]
struct GameState {
	board: Board,
	turn: Player,
	round_count: usize,
}

impl GameState {
	pub fn new() -> Self {
		Self {
			board: [None, None, None, None, None, None, None, None, None],
			turn: Player::CROSS,
			round_count: 1,
		}
	}
}

impl fmt::Display for GameState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}{}{}\n{}{}{}\n{}{}{}\n",
			disp(self.board[0]),
			disp(self.board[1]),
			disp(self.board[2]),
			disp(self.board[3]),
			disp(self.board[4]),
			disp(self.board[5]),
			disp(self.board[6]),
			disp(self.board[7]),
			disp(self.board[8]),
		)
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
	CROSS,
	NOUGHT,
}

impl Player {
	fn flip(&self) -> Self {
		if *self == Player::CROSS {
			return Player::NOUGHT;
		} else {
			return Player::CROSS;
		}
	}
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Player::CROSS => write!(f, "X"),
			Player::NOUGHT => write!(f, "O"),
		}
	}
}

fn disp(op: Option<Player>) -> String {
	format!("{}", op.map_or("_".to_string(), |x| x.to_string()))
}

type Board = [Option<Player>; 9];

fn board_set_piece(mut b: Board, x: usize, y: usize, p: Player) -> Board {
	b[x % 3 + y * 3] = Some(p);
	b
}
