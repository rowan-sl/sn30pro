//! # SN30pro
//!
//! A simple library for interfacing with the 8BitDo SN30pro controller on linux
//! (and linux only)
//!
//! this should also work with a xbox360 controller with some modifications
//! (L2 and R2 are not buttons with the xbox 360), as the SN30 identifies itself as one
//! to the computer, and probably uses the same api
//!
//! ## Platform support
//! linux only, and there are no plans to expand this, as this is mostly a learning project for me
//!
//! ## Getting Started
//!
//! the base of the library is the [`Controller`], which is initialized with 
//! the id of the controller you want to connect to (corisponds to `/dev/input/js{}`)
//!
//! this library uses async io, so you need a async executor to use it
//!
//! ```
//! use sn30pro::Controller;
//!
//! let mut sn30: Controller = Controller::new(0).await?;// for the controller at /dev/input/js0
//! ```
//! 

pub mod raw;
pub mod base;
pub mod parts;
pub mod controller;

pub use controller::Controller;