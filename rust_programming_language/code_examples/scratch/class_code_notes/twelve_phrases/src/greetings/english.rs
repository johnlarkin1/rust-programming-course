//! This module contains English phrases.
//! 
//! # Examples 
//! ```
//! let username = "John";
//! println!("{}, {}!", 
//!     phrases:greetings::english::hello(),
//!     username
//! );
//! ```

/// Applies to code that follows it.
/// In this case, it's our `hello()` function.
pub fn hello() -> String { "hello".to_string() }
pub fn goodbye() -> String { "goodbye".to_string() }