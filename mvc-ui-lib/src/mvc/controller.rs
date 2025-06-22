//!
//! Rust MVC-UI
//!
use crate::control::Control;

///
/// The Controller
/// 
pub trait Controller {

    fn view(&self) -> dyn Control;

}