#[allow(non_camel_case_types)]
pub type IMPL_T<A> = Option<A>;

pub pure fn EACH<A>(self: &IMPL_T<A>, f: fn(v: &A) -> bool) {
    match *self {
      None => (),
      Some(ref a) => { f(a); }
    }
}

pub pure fn SIZE_HINT<A>(self: &IMPL_T<A>) -> Option<uint> {
    match *self {
      None => Some(0),
      Some(_) => Some(1)
    }
}
