#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub(crate) struct Foo;

trait RefCall: Sized {
    fn call(self);
}

#[rustversion_msrv::msrv]
impl RefCall for Foo {
    fn call(self) {
        // is msrv
    }
}

impl RefCall for &Foo {
    fn call(self) {
        panic!("not MSRV")
    }
}

#[allow(unused_imports)]
#[test]
fn test() {
    Foo.call();
}
