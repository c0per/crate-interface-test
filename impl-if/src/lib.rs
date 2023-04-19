use crate_interface::impl_interface;
use def_call::If;

struct IfImpl;

#[impl_interface]
impl If for IfImpl {
    fn hello(s: &str) {
        println!("Hello {}", s);
    }
}

pub fn dummy_fn_in_impl_if() {}
