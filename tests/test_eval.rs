mod always {
    pub(super) fn success() {
        panic!("not MSRV")
    }
}

mod sometimes {
    #[rustversion_msrv::msrv]
    pub(super) fn success() {
        // is MSRV
    }
}

#[allow(unused_imports)]
#[test]
fn test() {
    use self::{always::*, sometimes::*};

    success();
}
