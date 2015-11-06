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
    // boehm_gc::use_threads_discovery();
    // unsafe { boehm_gc::gc_allow_register_threads(); }
    // ::std::thread::at_start(|| { unsafe { boehm_gc::gc_register_myself(); } });

    let tests = vec![test!(hello_gc_alloc)];
    let args: Vec<_> = env::args().collect();
    test::test_main(&args[..], tests);
}

fn hello_gc_alloc() {
    use boehm_gc::{gc_allocate};
    const BYTES: usize = 408;
    println!("Hello gc_allocate {} bytes", BYTES);
    for _i in 0..1000 {
        let mut _c;
        for _l in 0..1000 {
            _c = gc_allocate(BYTES);
        }
    }
}
