#[rustversion_msrv::nightly(stable)]
struct S;

#[rustversion_msrv::any(nightly(stable))]
struct S;

fn main() {}
