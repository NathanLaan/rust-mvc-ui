//!
//! Rust MVC-UI
//!

mod button;

///
/// Root GUI Control.
///
pub trait Control {

    ///
    /// Parent control.
    ///
    fn parent() -> Self where Self: Sized;

    ///
    /// Child controls.
    ///
    fn children() -> Vec<Self> where Self: Sized;

}