#![feature(test, TODO_unnamed_feature)]

extern crate boehm_gc;
extern crate test;

use std::env;
macro_rules! test {
    ($id:ident) => {
        test::TestDescAndFn { desc: test::TestDesc { name: test::TestName::StaticTestName(stringify!($id)),
                                                     ignore: false,
                                                     should_panic: test::ShouldPanic::No },
                        testfn: test::TestFn::StaticTestFn($id) }
    }
}
fn main() {
    // This does not work with DARWIN_DONT_PARSE_STACK, which seems to be turned on
    // by default (despite what the documentation implies).
    //
    // boehm_gc::use_threads_discovery();

    // This works but is unnnecessary with our own private build of libgc, which
    // has GC_ALWAYS_MULTITHREADED turned on when it is built.
    //
    // unsafe { boehm_gc::gc_allow_register_threads(); }

    // This is probably what we want to encourage others to do, but it requires
    // std::thread::at_start, which is not part of Rust.
    // I started a discussion of this at:
    //   https://internals.rust-lang.org/t/rfrfc-std-thread-at-start-callback/2877
    // but who knows if it will go anywhere.
    // For now, you can emulate the effect of this by manually invoking `gc_register_myself`
    // at the start of each newly spawned thread.
    //
    // ::std::thread::at_start(|| { unsafe { boehm_gc::gc_register_myself(); } });

    let tests = vec![test!(hello_gc_alloc)];
    let args: Vec<_> = env::args().collect();
    test::test_main(&args[..], tests);
}

fn hello_gc_alloc() {
    use boehm_gc::{gc_allocate};
    unsafe { boehm_gc::gc_register_myself(); }
    const BYTES: usize = 408;
    println!("Hello gc_allocate {} bytes", BYTES);
    for _i in 0..1000 {
        let mut _c;
        for _l in 0..1000 {
            _c = gc_allocate(BYTES);
        }
    }
}
