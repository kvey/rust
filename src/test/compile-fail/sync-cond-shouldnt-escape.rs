// error-pattern: reference is not valid outside of its lifetime
extern mod std;
use std::sync;

fn main() {
    let m = ~sync::Mutex();
    let mut cond = None;
    do m.lock_cond |c| {
        cond = Some(c);
    }   
    option::unwrap(cond).signal();
}
