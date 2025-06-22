//!
//! Rust MVC-UI
//!

use std::cell::RefCell;
use std::rc::{Rc, Weak};

///
/// Event holds a list of Callback<handler> references.
/// 
pub struct Event<Handler> {
    event_callback_list: RefCell<Vec<Weak<Callback<Handler>>>>,
}

#[derive(Clone)]
pub struct Callback<Handler> {
    callback: Rc<RefCell<Option<Box<dyn 'static + FnMut(Handler)>>>>,
}