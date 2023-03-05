pub mod hosting;

// Making a module public doesn't make the content
// of that model public. The pub in a module only
// makes that the module is reachable by his ancestor
// modules. If we can access the parent module of a pub,
// we can also access to the pub module.

pub mod entrance {
    pub fn give_entrance() {}
}

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}
