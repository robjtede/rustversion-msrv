#[rustversion_msrv::any(not)]
struct S;

#[rustversion_msrv::any(not, not)]
struct S;

fn main() {}
