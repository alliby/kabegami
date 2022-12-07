//! A OS and desktop environment indipendent get/set wallpaper library.
//!
//! # Comparison with similiar libraries
//!
//! [wallpaper](https://crates.io/crates/wallpaper) and [wallpaper_rs](https://crates.io/crates/wallpaper_rs) both don't have a decent Linux implementation.  
//! They are both hacky and support only some desktop environments/window managers.
//! This library uses directly Xlib for the Linux part, so if you are running Xorg,
//! it works.  
//!
//! # Help wanted
//!
//! Help is wanted for the MacOS part. I don't have a Mac machine so I can't test
//! the code. The current implementation is a bit hacky and I would like to get rid
//! of it.  
//! An idea would be to port [this
//! code](https://github.com/sindresorhus/macos-wallpaper/blob/master/Sources/wallpaper/Wallpaper.swift).
//!
//! # Todo
//!
//! - [x] Xorg
//! - [x] Windows
//! - [x] MacOS (hacky)
//! - [ ] Wayland
//! - [ ] RedoxOS
//! - [ ] Better MacOS implementation
//!
//! ### License
//!
//! <sup>
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//! </sup>
//!
//! <br>
//!
//! <sub>
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.
//! </sub>

#[cfg(target_os = "linux")]
pub mod xlib;

#[cfg(windows)]
pub mod windows;
