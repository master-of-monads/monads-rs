#![feature(prelude_import)]
#![cfg(feature = "state")]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use monads_rs::state::State;
use monads_rs::*;
use std::fmt;
fn main() {
	let state = program().run(GameState::new());
	{
		::std::io::_print(format_args!("{0}\n", state.0));
	};
}
type GameStateState<A> = State<'static, GameState, A>;
fn program() -> GameStateState<()> {
	::monads_rs::Monad::bind(play_turn(0, 0), move |__monads_rs_temp_ident_0| {
		__monads_rs_temp_ident_0;
		::monads_rs::Monad::bind(
			play_turn(0, 2),
			move |__monads_rs_temp_ident_0| {
				__monads_rs_temp_ident_0;
				::monads_rs::Monad::bind(
					play_turn(2, 0),
					move |__monads_rs_temp_ident_0| {
						__monads_rs_temp_ident_0;
						::monads_rs::Monad::bind(
							play_turn(1, 0),
							move |__monads_rs_temp_ident_0| {
								__monads_rs_temp_ident_0;
								::monads_rs::Monad::bind(
									play_turn(2, 2),
									move |__monads_rs_temp_ident_0| {
										__monads_rs_temp_ident_0;
										::monads_rs::Monad::bind(
											play_turn(2, 1),
											move |__monads_rs_temp_ident_0| {
												__monads_rs_temp_ident_0;
												play_turn(1, 1)
											},
										)
									},
								)
							},
						)
					},
				)
			},
		)
	})
}
fn play_turn(x: usize, y: usize) -> GameStateState<()> {
	::monads_rs::Monad::bind(set_piece(x, y), move |__monads_rs_temp_ident_0| {
		__monads_rs_temp_ident_0;
		::monads_rs::Monad::bind(
			switch_current_player(),
			move |__monads_rs_temp_ident_0| {
				__monads_rs_temp_ident_0;
				inc_round_counter()
			},
		)
	})
}
fn set_piece(x: usize, y: usize) -> GameStateState<()> {
	::monads_rs::Monad::bind(get_board(), move |__monads_rs_temp_ident_0| {
		let board = __monads_rs_temp_ident_0;
		::monads_rs::Monad::bind(
			get_current_player(),
			move |__monads_rs_temp_ident_0| {
				let player = __monads_rs_temp_ident_0;
				set_board(board_set_piece(board, x, y, player))
			},
		)
	})
}
fn get_board() -> GameStateState<Board> {
	::monads_rs::Monad::bind(
		GameStateState::<GameState>::get(),
		move |__monads_rs_temp_ident_0| {
			let state = __monads_rs_temp_ident_0;
			State::ret(state.board)
		},
	)
}
fn get_current_player() -> GameStateState<Player> {
	::monads_rs::Monad::bind(
		GameStateState::<GameState>::get(),
		move |__monads_rs_temp_ident_0| {
			let state = __monads_rs_temp_ident_0;
			State::ret(state.turn)
		},
	)
}
fn set_board(board: Board) -> GameStateState<()> {
	::monads_rs::Monad::bind(
		GameStateState::<GameState>::get(),
		move |__monads_rs_temp_ident_0| {
			let mut state = __monads_rs_temp_ident_0;
			state.board = board;
			GameStateState::<()>::set(state)
		},
	)
}
fn switch_current_player() -> GameStateState<()> {
	::monads_rs::Monad::bind(
		GameStateState::<GameState>::get(),
		move |__monads_rs_temp_ident_0| {
			let mut state = __monads_rs_temp_ident_0;
			state.turn = state.turn.flip();
			GameStateState::<()>::set(state)
		},
	)
}
fn inc_round_counter() -> GameStateState<()> {
	::monads_rs::Monad::bind(
		GameStateState::<GameState>::get(),
		move |__monads_rs_temp_ident_0| {
			let mut state = __monads_rs_temp_ident_0;
			state.round_count += 1;
			return GameStateState::<()>::set(state);
		},
	)
}
struct GameState {
	board: Board,
	turn: Player,
	round_count: usize,
}
#[automatically_derived]
impl ::core::clone::Clone for GameState {
	#[inline]
	fn clone(&self) -> GameState {
		let _: ::core::clone::AssertParamIsClone<Board>;
		let _: ::core::clone::AssertParamIsClone<Player>;
		let _: ::core::clone::AssertParamIsClone<usize>;
		*self
	}
}
#[automatically_derived]
impl ::core::marker::Copy for GameState {}
#[automatically_derived]
impl ::core::fmt::Debug for GameState {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		::core::fmt::Formatter::debug_struct_field3_finish(
			f,
			"GameState",
			"board",
			&self.board,
			"turn",
			&self.turn,
			"round_count",
			&&self.round_count,
		)
	}
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
		f.write_fmt(format_args!(
			"{0}{1}{2}\n{3}{4}{5}\n{6}{7}{8}\n",
			disp(self.board[0]),
			disp(self.board[1]),
			disp(self.board[2]),
			disp(self.board[3]),
			disp(self.board[4]),
			disp(self.board[5]),
			disp(self.board[6]),
			disp(self.board[7]),
			disp(self.board[8])
		))
	}
}
enum Player {
	CROSS,
	NOUGHT,
}
#[automatically_derived]
impl ::core::clone::Clone for Player {
	#[inline]
	fn clone(&self) -> Player {
		*self
	}
}
#[automatically_derived]
impl ::core::marker::Copy for Player {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Player {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Player {
	#[inline]
	fn eq(&self, other: &Player) -> bool {
		let __self_tag = ::core::intrinsics::discriminant_value(self);
		let __arg1_tag = ::core::intrinsics::discriminant_value(other);
		__self_tag == __arg1_tag
	}
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Player {}
#[automatically_derived]
impl ::core::cmp::Eq for Player {
	#[inline]
	#[doc(hidden)]
	#[no_coverage]
	fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::fmt::Debug for Player {
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		::core::fmt::Formatter::write_str(
			f,
			match self {
				Player::CROSS => "CROSS",
				Player::NOUGHT => "NOUGHT",
			},
		)
	}
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
			Player::CROSS => f.write_fmt(format_args!("X")),
			Player::NOUGHT => f.write_fmt(format_args!("O")),
		}
	}
}
fn disp(op: Option<Player>) -> String {
	{
		let res = ::alloc::fmt::format(format_args!(
			"{0}",
			op.map_or("_".to_string(), |x| x.to_string())
		));
		res
	}
}
type Board = [Option<Player>; 9];
fn board_set_piece(mut b: Board, x: usize, y: usize, p: Player) -> Board {
	b[x % 3 + y * 3] = Some(p);
	b
}
