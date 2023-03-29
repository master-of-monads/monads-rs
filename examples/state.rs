use std::fmt;

use monads_rs::*;

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

#[monadic]
fn inc_round_counter() -> State<'static, GameState, ()> {
	let mut state = State::<'_, GameState, GameState>::get()?;
	state.round_count += 1;
	return State::<'_, GameState, ()>::set(state);
}

// Getters
#[monadic]
fn get_board() -> State<'static, GameState, Board> {
	let state = State::<'_, GameState, GameState>::get()?;
	State::ret(state.board)
}

#[monadic]
fn get_current_player() -> State<'static, GameState, Player> {
	let state = State::<'_, GameState, GameState>::get()?;
	State::ret(state.turn)
}

// Setters

#[monadic]
fn set_board(board: Board) -> State<'static, GameState, ()> {
	State::<'_, GameState, GameState>::get().bind(move |mut state| {
		state.board = board;
		State::<'_, GameState, ()>::set(state)
	})
}

#[monadic]
fn switch_current_player() -> State<'static, GameState, ()> {
	let mut state = State::<'_, GameState, GameState>::get()?;
	state.turn = state.turn.flip();
	State::<'_, GameState, ()>::set(state)
}

#[monadic]
fn set_piece(x: usize, y: usize) -> State<'static, GameState, ()> {
	get_board().bind(move |board| {
		get_current_player().bind(move |player| {
			let mut new_board = board.clone();
			new_board[x % 3 + y * 3] = Some(player);
			set_board(new_board)
		})
	})
}

#[monadic]
fn play_turn(x: usize, y: usize) -> State<'static, GameState, ()> {
	set_piece(x, y)?;
	switch_current_player()?;
	inc_round_counter()
}

#[monadic]
fn program() -> State<'static, GameState, ()> {
	play_turn(0, 0)?;
	play_turn(0, 2)?;
	play_turn(2, 0)?;
	play_turn(1, 0)?;
	play_turn(2, 2)?;
	play_turn(2, 1)?;
	play_turn(1, 1)
}

fn main() {
	let state = program().run(GameState::new());
	println!("{}", state.0);
}
