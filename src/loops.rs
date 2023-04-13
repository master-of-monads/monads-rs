use super::Monad;

pub fn bind_for_loop<'m, C, M, NextM>(
	container: C,
	body: impl FnMut(C::Item) -> M + 'm,
) -> NextM
where
	C: IntoIterator,
	C::IntoIter: Clone + 'm,
	M: Monad<'m, (), Bind<()> = NextM>,
	NextM: Monad<'m, ()>,
{
	let iterator = container.into_iter();
	recurse_for_loop(iterator, body)
}

fn recurse_for_loop<'m, I, F, M, NextM>(mut iterator: I, mut body: F) -> NextM
where
	I: Iterator + Clone + 'm,
	F: FnMut(I::Item) -> M + 'm,
	M: Monad<'m, (), Bind<()> = NextM>,
	NextM: Monad<'m, ()>,
{
	if let Some(item) = iterator.next() {
		let result = body(item);
		result.bind::<(), _>(move |_| {
			let body: F = unsafe { std::mem::transmute_copy(&body) };
			recurse_for_loop(iterator.clone(), body)
		})
	} else {
		NextM::ret(())
	}
}

pub fn bind_while_loop<'m, C, F, MBool, M, NextM>(
	mut condition: C,
	body: F,
) -> NextM
where
	C: FnMut() -> MBool + 'm,
	F: FnMut() -> M + 'm,
	MBool: Monad<'m, bool, Bind<()> = NextM>,
	M: Monad<'m, (), Bind<()> = NextM>,
	NextM: Monad<'m, ()>,
{
	condition().bind::<(), _>(move |c| {
		if c {
			let mut body: F = unsafe { std::mem::transmute_copy(&body) };
			let condition: C = unsafe { std::mem::transmute_copy(&condition) };
			body().bind::<(), _>(move |_| {
				let body: F = unsafe { std::mem::transmute_copy(&body) };
				let condition: C =
					unsafe { std::mem::transmute_copy(&condition) };
				bind_while_loop(condition, body)
			})
		} else {
			NextM::ret(())
		}
	})
}
