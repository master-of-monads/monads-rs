use criterion::{
	black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use monads_rs::state::State;
use monads_rs::*;

#[derive(Clone, Copy)]
struct GameState {
	x_position: usize,
	y_position: usize,
	score: f32,
}

impl GameState {
	pub fn get_x(&self) -> usize {
		return self.x_position;
	}
	pub fn get_y(&self) -> usize {
		return self.y_position;
	}
	pub fn inc_y(&self, val: usize) -> Self {
		GameState {
			x_position: self.x_position,
			y_position: self.y_position + val,
			score: self.score,
		}
	}
	pub fn inc_x(&self, val: usize) -> Self {
		GameState {
			x_position: self.x_position + val,
			y_position: self.y_position,
			score: self.score,
		}
	}
	pub fn set_score(&self, val: f32) -> Self {
		GameState {
			x_position: self.x_position,
			y_position: self.y_position,
			score: val,
		}
	}
}

fn non_monadic(s: GameState) -> GameState {
	let s = s.inc_x(10).inc_y(10);
	let s = s.set_score((s.get_x() + s.get_y()) as f32);
	s
}

#[monadic]
fn set_score(v: f32) -> State<'static, GameState, ()> {
	let state: GameState = State::<'_, GameState, GameState>::get()?;
	State::<'_, GameState, GameState>::set(state.set_score(v))
}

#[monadic]
fn inc_x(v: usize) -> State<'static, GameState, ()> {
	let state = State::<'_, GameState, GameState>::get()?;
	State::<'_, GameState, GameState>::set(state.inc_x(v))
}

#[monadic]
fn inc_y(v: usize) -> State<'static, GameState, ()> {
	let state = State::<'_, GameState, GameState>::get()?;
	State::<'_, GameState, GameState>::set(state.inc_y(v))
}

#[monadic]
fn get_x() -> State<'static, GameState, usize> {
	let state = State::<'_, GameState, GameState>::get()?;
	State::<'_, GameState, usize>::ret(state.get_x())
}

#[monadic]
fn get_y() -> State<'static, GameState, usize> {
	let state = State::<'_, GameState, usize>::get()?;
	State::ret(state.get_y())
}

#[monadic]
fn monadic() -> State<'static, GameState, ()> {
	inc_x(10)?;
	inc_y(10)?;
	let x = get_x()?;
	let y = get_y()?;
	set_score((x + y) as f32)
}

pub fn criterion_benchmark(c: &mut Criterion) {
	let mut group = c.benchmark_group("State");

	let initial_state = GameState {
		x_position: 0,
		y_position: 0,
		score: 0.0,
	};
	for i in [250].iter() {
		group.bench_with_input(
			BenchmarkId::new("Monadic State", i),
			i,
			|b, i| b.iter(|| monadic().run(initial_state)),
		);
		group.bench_with_input(
			BenchmarkId::new("Refrence State", i),
			i,
			|b, i| b.iter(|| non_monadic(black_box(initial_state))),
		);
	}
	group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
