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
