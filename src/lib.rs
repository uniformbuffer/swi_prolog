#![allow(dead_code)]

//! This crate wrap the Swi Prolog engine into a more rusty interface.
//! This is absolutely an amateur project and there is no relation to Swi Prolog developers.
//! The reason for calling the crate in this way is because it link to C interface defined in SWI-Prolog.h file, often shipped with `swipl` binary.
//! Also keep in mind that this crate very very alpha and since it link to a C interface, it obviously use unsafe functions.
//! Even if i have tried to stick as much as possible to the documentation, there is still the risk of crash, so do not use it in critical systems.
//! Lastly the binding are generated using `bindgen`, but since i need to modify them a bit, the crate ship a customized `bindgen` sys interface.
//! This means that the crate do not require `bindgen`, but keep in mind that has been used and, if you can, support them because it is a great project (https://github.com/rust-lang/rust-bindgen).

extern crate lazy_static;
extern crate tokio;


mod bindings;
mod data;
mod predicate;
mod query;
mod swi_prolog;
mod module;
mod term;
mod frame;
mod functor;
mod engine;

pub use swi_prolog::SwiProlog;
pub use data::Data;
pub use term::Term;

#[cfg(test)]
mod test;

#[cfg(logger)]
mod logger;

