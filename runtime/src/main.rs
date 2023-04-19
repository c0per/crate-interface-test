use def_call::remote_call;

// This works with extern crate:
// extern crate impl_if;
//
// fn main() {
//     remote_call("from runtime calling def-call with implementation in impl-if");
// }

// This works with usage from the dep:
fn main() {
    use impl_if::dummy_fn_in_impl_if;
    dummy_fn_in_impl_if();
    remote_call("from runtime calling def-call with implementation in impl-if");
}

// This works with unused_imports warning:
// fn main() {
//     use impl_if::dummy_fn_in_impl_if;
//     remote_call("from runtime calling def-call with implementation in impl-if");
// }



// This doesn't work.
// Seems like rust "removes" unused dependency although listed in Cargo.toml
// fn main() {
//     remote_call("from runtime calling def-call with implementation in impl-if");
// }
