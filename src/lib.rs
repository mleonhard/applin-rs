//! Maggie
//! ======
//! [![crates.io version](https://img.shields.io/crates/v/maggie.svg)](https://crates.io/crates/maggie)
//! [![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/maggie-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/maggie-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://github.com/mleonhard/maggie-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/maggie-rs/actions)
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
//!   http server for good performance under load. - TODO: Make a benchmark.
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
//!   - [Docs are not searchable](https://github.com/apple/swift-org-website/issues/24)
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
//!       no detail cell widget or check box (Flutter Team replied to my email and refused to add these),
//!       [broken dark mode support](https://github.com/flutter/flutter/issues/80860),
//!       [no scroll-to-dismiss keyboard behavior](https://github.com/flutter/flutter/issues/57609) which is used by most iOS apps,
//!       [no keyboard dismiss button](https://github.com/flutter/flutter/issues/45076) which is used by all the other iOS apps,
//!     - Documentation has large holes:
//!       [Navigator](https://github.com/flutter/flutter/issues/69315),
//!       [focus management](https://github.com/flutter/flutter/issues/45076),
//!       non-trivial examples, local data management
//!     - [Integration test support is broken & deprecated](https://github.com/flutter/flutter/issues?q=is%3Aissue+author%3Amleonhard+integration+test+).
//!     - Camera & photo module has a [show-stopping bug](https://github.com/flutter/flutter/issues/70751), ignored for 3 years
//!     - [No high-quality location module](https://github.com/flutter/flutter/issues/31453)
//!     - Debugger lacks async support
//!     - [UI inspection tools are buggy](https://github.com/flutter/flutter-intellij/issues/4426), not fixed in 3 years
//!     - When your Dart code times out waiting for an HTTP request,
//!       [the request continues in the background](https://github.com/dart-lang/http/issues/424),
//!       wasting mobile device CPU & RAM, data transfer, battery, and server resources.
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
#![forbid(unsafe_code)]

mod action;
mod action_builders;
mod back_button;
mod button;
mod column;
mod context;
mod context_set;
mod detail_cell;
mod error;
mod h_alignment;
mod key_set;
mod nav_page;
mod page_enum;
mod plain_page;
mod random;
mod roster;
mod session_cookie;
mod session_id;
mod session_set;
mod session_struct;
mod text;
mod v_alignment;
mod widget_builders;
mod widget_enum;
mod widget_list;

pub mod reexports {
    pub use serde_json;
}

pub mod data {
    pub use crate::context::*;
    pub use crate::context_set::*;
    pub use crate::error::*;
    pub use crate::random::*;
    pub use crate::roster::*;
}

pub mod session {
    pub use crate::session_cookie::*;
    pub use crate::session_id::*;
    pub use crate::session_set::*;
    pub use crate::session_struct::*;
}

pub mod page {
    pub use crate::key_set::*;
    pub use crate::page_enum::*;
}

pub mod builder {
    pub use crate::action_builders::*;
    pub use crate::back_button::*;
    pub use crate::button::*;
    pub use crate::column::*;
    pub use crate::detail_cell::*;
    pub use crate::nav_page::*;
    pub use crate::plain_page::*;
    pub use crate::text::*;
    pub use crate::widget_builders::*;
}

pub mod widget {
    pub use crate::action::*;
    pub use crate::h_alignment::*;
    pub use crate::v_alignment::*;
    pub use crate::widget_enum::*;
    pub use crate::widget_list::*;
}
