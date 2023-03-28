use super::{Applicative, Functor, Monad};
use std::{rc::Rc, marker::PhantomData};


#[derive(Clone)]
struct State<'a, S,V> {
    runState: &'a (dyn Fn(S) -> V),
}

impl<'a,S:'a,A:'a> Functor<'a,A> for State<S,A> {
	type Map<B> = State<S,B>;

	fn map<B, F: Fn(A) -> B>(self, f: &'static F) -> Self::Map<B> {
        let rs = self.runState.clone();
        let new_rs = move |s| f(rs.as_ref()(s));
        return Self::Map::<B>{runState : Rc::new(new_rs)}
	}
}

