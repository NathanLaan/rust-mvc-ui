//!
//!
//!

///
/// Root GUI Control.
///
pub trait Control {
    
    ///
    /// Child controls.
    /// 
    fn children() -> Vec<Self> where Self: Sized;

}