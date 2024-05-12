#[rustversion_msrv::stable(nightly)]
struct S;

#[rustversion_msrv::any(stable(nightly))]
struct S;

fn main() {}
