use crate_interface::{def_interface, call_interface};

#[def_interface]
pub trait If {
    fn hello(s: &str);
}

pub fn remote_call(s: &str) {
    call_interface!(If::hello, s);
}
