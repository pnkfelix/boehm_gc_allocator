extern crate boehm_gc;

use boehm_gc::{Gc, heap_size};
use std::mem;

fn main() {
    test_box_chains();
    test_gc_chains();
}

// Quick-and-dirty demo linked-lists

/// `BoxChain` is a `Box`-based linked list. Each cell owns the next. It
/// (and its children) will be reclaimed immediately when it goes out
/// of scope (or is otherwise dropped).
struct BoxChain<T>(T, Option<Box<BoxChain<T>>>);

/// `GcChain` is a Gc-based linked list. Each cell shares ownership of
/// the next. It relies on the collector to reclaim the cells sometime
/// after they become globally unreachable.
struct GcChain<T>(T, Option<Gc<GcChain<T>>>);

// Avoid stack-overflow errors for long chains by doing the necessary
// dropping in a loop instead.
impl<T> Drop for BoxChain<T> {
    fn drop(&mut self) {
        let mut next = mem::replace(&mut self.1, None);
        while let Some(mut n) = next {
            next = mem::replace(&mut n.1, None);
        }
    }
}

/// Builds a list of length `count`, each cell holding a copy of `t`.
fn box_chain<T:Clone>(t: T, mut count: usize) -> Option<Box<BoxChain<T>>> {
    let mut r = None;
    while count > 0 { r = Some(Box::new(BoxChain(t.clone(), r))); count -= 1; }
    r
}

/// Builds a list of length `count`, each cell holding a copy of `t`.
fn gc_chain<T:Clone>(t: T, mut count: usize) -> Option<Gc<GcChain<T>>> {
    let mut r = None;
    while count > 0 { r = Some(Gc::new(GcChain(t.clone(), r))); count -= 1; }
    r
}

// These macros form the body of the test functions that exercise the above
// two chain variants.

macro_rules! chain_body {
    ($iter:expr, $len:expr, $chain_fn:ident) => { { {
        const E: usize = $len;
        #[derive(Copy)]
        struct Data([i32; E]);
        impl Clone for Data { fn clone(&self) -> Self { *self } }
        // let a = [$iter; E];
        let a = Data([3; E]);
        for &l in &CHAIN_LENGTHS {
            // println!("chain {} of [3; {}]", l, E);
            let mut _c = $chain_fn(a, l);
            _c = $chain_fn(a, 0);
        }
    } } }
}

macro_rules! chain_fn {
    ($name: ident, $chain_fn:ident, $($elem_len:expr),*) => {
        fn $name() {
            const NAME: &'static str = stringify!($chain_fn);
            println!("{}", stringify!($name));
            let mut i: usize = 1;
            println!("{:>10} {:10} {:5} {:11}",
                     NAME, "start", i, "heap_size()");
            while i < ITERS {
                if i < 5 || i % 100 == 0 { println!("{:>10} {:10} {:5} {:11}",
                                                    NAME, "iteration", i, heap_size()); }
                i += 1;
                $(
                    chain_body!(i, $elem_len, $chain_fn);
                    )*
            }
            println!("{:>10} {:10} {:5} {:11}",
                     NAME, "finished", i, heap_size());
        }
    }
}

const CHAIN_LENGTHS: [usize; 4] = [10, 100, 1_000, 10_000];
const ITERS: usize = 1000;

chain_fn!(test_box_chains, box_chain, 8, 32, 128);
chain_fn!(test_gc_chains, gc_chain, 8, 32, 128);
