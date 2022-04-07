//! Maggie
//! ======
//! [![crates.io version](https://img.shields.io/crates/v/beatrice.svg)](https://crates.io/crates/beatrice)
//! [![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/beatrice-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/beatrice-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://github.com/mleonhard/beatrice-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/beatrice-rs/actions)
//!
//! An app server library in Rust.
//!
//! Use a Maggie client to access the app:
//! - `maggie-ios`
//! - `maggie-android`
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - Define your app's UI and RPCs in threaded Rust.
//! - Uses [`beatrice`](https://github.com/mleonhard/beatrice-rs)
//!   http server for good performance under load.
//! - No macros or complicated type params
//! - Good test coverage (??%) - TODO: Update.
//!
//! # Limitations
//! - New, not proven in production.
//!
//! # Examples
//! Complete examples: [`examples/`](examples/).
//!
//! Simple example:
//! ```rust
//! ```
//! # Cargo Geiger Safety Report
//! # Alternatives
//!
//! - [React Native](https://reactnative.dev/)
//!   - Very popular
//!   - iOS & Android
//!   - Write apps in TypeScript or JavaScript
//!   - Immediately deploy new versions
//!   - Develop on Linux, macOS, & Windows
//!   - Toolchain is diverse and not cohesive.
//! - [Xcode](https://developer.apple.com/xcode/)
//!   - Very popular
//!   - iOS
//!   - Write apps in Swift or Objective-C
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on macOS only
//! - [Android Studio](https://developer.android.com/studio/)
//!   - Very popular
//!   - Android
//!   - Write apps in Kotlin or Java
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on Linux, macOS, Windows, & Chrome OS
//! - [Xamarin](https://dotnet.microsoft.com/en-us/apps/xamarin)
//!   - iOS & Android
//!   - Write apps in C# and other .Net language
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on Linux, macOS, & Windows
//! - [Flutter](https://flutter.dev/)
//!   - iOS & Android
//!   - Web version has unusable performance
//!   - Write apps in async Dart
//!   - Good package management system
//!   - Compiled code is very small
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on Linux, macOS, & Windows
//!   - Widgets look and behave similar to native widgets
//!   - Unmaintained - little progress on mobile features since 2019
//!     - iOS support is incomplete:
//!       no detail cell widget, no check box, broken dark mode support, no keyboard dismiss button
//!     - Documentation has large holes: Navigator, focus management, non-trivial examples, local data management
//!     - No integration test support
//!     - Camera & photo module has show-stopping bug unaddressed for 3 years
//!     - No high-quality location module
//!     - Debugger lacks async support
//!     - UI inspection tools are buggy, not fixed in 3 years
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
#![forbid(unsafe_code)]
pub mod error;
pub mod random;
pub mod rebuilder;
pub mod rebuilder_map;
pub mod rebuilder_set;
pub mod session;
pub mod session_cookie;
pub mod session_id;
pub mod session_token;
pub mod widgets;
