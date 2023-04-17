use super::Monad;

pub fn bind_for_loop<'m, C, M, NextM, FinalM>(
	container: C,
	body: impl FnMut(C::Item) -> M + 'm,
) -> FinalM
where
	C: IntoIterator,
	C::IntoIter: Clone + 'm,
	M: Monad<'m, LoopControl, Bind<LoopControl> = NextM>,
	NextM: Monad<'m, LoopControl, Bind<()> = FinalM>,
	FinalM: Monad<'m, ()>,
{
	let iterator = container.into_iter();
	recurse_for_loop(iterator, body).bind::<(), _>(|_| FinalM::ret(()))
}

fn recurse_for_loop<'m, I, F, M, NextM>(mut iterator: I, mut body: F) -> NextM
where
	I: Iterator + Clone + 'm,
	F: FnMut(I::Item) -> M + 'm,
	M: Monad<'m, LoopControl, Bind<LoopControl> = NextM>,
	NextM: Monad<'m, LoopControl>,
{
	if let Some(item) = iterator.next() {
		let result = body(item);
		result.bind::<LoopControl, _>(move |control| match control {
			LoopControl::Break() => NextM::ret(LoopControl::Break()),
			LoopControl::Continue() => {
				let body: F = unsafe { std::mem::transmute_copy(&body) };
				recurse_for_loop(iterator.clone(), body)
			}
		})
	} else {
		NextM::ret(LoopControl::Break())
	}
}

pub fn bind_while_loop<'m, C, F, MBool, M, NextM, FinalM>(
	condition: C,
	body: F,
) -> FinalM
where
	C: FnMut() -> MBool + 'm,
	F: FnMut() -> M + 'm,
	MBool: Monad<'m, bool, Bind<LoopControl> = NextM>,
	M: Monad<'m, LoopControl, Bind<LoopControl> = NextM>,
	NextM: Monad<'m, LoopControl, Bind<()> = FinalM>,
	FinalM: Monad<'m, ()>,
{
	recurse_while_loop(condition, body).bind::<(), _>(|_| FinalM::ret(()))
}

fn recurse_while_loop<'m, C, F, MBool, M, NextM>(
	mut condition: C,
	body: F,
) -> NextM
where
	C: FnMut() -> MBool + 'm,
	F: FnMut() -> M + 'm,
	MBool: Monad<'m, bool, Bind<LoopControl> = NextM>,
	M: Monad<'m, LoopControl, Bind<LoopControl> = NextM>,
	NextM: Monad<'m, LoopControl>,
{
	condition().bind::<LoopControl, _>(move |c| {
		if c {
			let mut body: F = unsafe { std::mem::transmute_copy(&body) };
			let condition: C = unsafe { std::mem::transmute_copy(&condition) };
			let result = body();
			result.bind::<LoopControl, _>(move |control| match control {
				LoopControl::Break() => NextM::ret(LoopControl::Break()),
				LoopControl::Continue() => {
					let body: F = unsafe { std::mem::transmute_copy(&body) };
					let condition: C =
						unsafe { std::mem::transmute_copy(&condition) };
					recurse_while_loop(condition, body)
				}
			})
		} else {
			NextM::ret(LoopControl::Break())
		}
	})
}

pub fn bind_loop_loop<'m, F, M, NextM, FinalM>(body: F) -> FinalM
where
	F: FnMut() -> M + 'm,
	M: Monad<'m, LoopControl, Bind<LoopControl> = NextM>,
	NextM: Monad<'m, LoopControl, Bind<()> = FinalM>,
	FinalM: Monad<'m, ()>,
{
	recurse_loop_loop(body).bind::<(), _>(|_| FinalM::ret(()))
}

pub fn recurse_loop_loop<'m, F, M, NextM>(mut body: F) -> NextM
where
	F: FnMut() -> M + 'm,
	M: Monad<'m, LoopControl, Bind<LoopControl> = NextM>,
	NextM: Monad<'m, LoopControl>,
{
	let result = body();
	result.bind::<LoopControl, _>(move |control| match control {
		LoopControl::Break() => NextM::ret(LoopControl::Break()),
		LoopControl::Continue() => {
			let body: F = unsafe { std::mem::transmute_copy(&body) };
			recurse_loop_loop(body)
		}
	})
}

#[derive(Clone, Copy)]
pub enum LoopControl {
	Break(),
	Continue(),
}
