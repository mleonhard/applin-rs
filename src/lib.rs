//! Applin
//! ======
//! [![crates.io version](https://img.shields.io/crates/v/applin.svg)](https://crates.io/crates/applin)
//! [![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/applin-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
//! [![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/applin-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![pipeline status](https://github.com/mleonhard/applin-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/applin-rs/actions)
//!
//! An app server library in Rust.
//!
//! Use an Applin client to access the app:
//! - [applin-ios](https://github.com/mleonhard/applin-ios)
//! - [applin-android](https://github.com/mleonhard/applin-android)
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - Define your app's UI and RPCs in threaded Rust.
//! - Uses [`servlin`](https://crates.io/crates/servlin)
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
//! // TODO: Add this.
//! ```
//! # Cargo Geiger Safety Report
//! # Alternatives
//!
//! - [React Native](https://reactnative.dev/)
//!   <details><summary>Details</summary>
//!
//!   - Very popular
//!   - iOS & Android
//!   - Write apps in TypeScript or JavaScript
//!   - Immediately deploy new versions
//!   - Develop on Linux, macOS, & Windows
//!   - Toolchain is diverse and not cohesive.
//!   </details>
//! - [Xcode](https://developer.apple.com/xcode/)
//!   <details><summary>Details</summary>
//!
//!   - Very popular
//!   - iOS
//!   - Write apps in Swift or Objective-C
//!   - Use `UIKit` or `SwiftUI` libraries
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on macOS only
//!   - [Docs are not searchable](https://github.com/apple/swift-org-website/issues/24)
//!   </details>
//! - [Android Studio](https://developer.android.com/studio/)
//!   <details><summary>Details</summary>
//!
//!   - Very popular
//!   - Android
//!   - Write apps in Kotlin or Java
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on Linux, macOS, Windows, & Chrome OS
//!   </details>
//! - [Xamarin](https://dotnet.microsoft.com/en-us/apps/xamarin)
//!   <details><summary>Details</summary>
//!
//!   - iOS & Android
//!   - Write apps in C# and other .Net language
//!   - Updates take multiple days to deploy.
//!   - Users choose when to update.
//!     Server must support multiple app versions or demand that users update.
//!   - Develop on Linux, macOS, & Windows
//!   </details>
//! - [Flutter](https://flutter.dev/)
//!   <details><summary>Details</summary>
//!
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
//!     - Debugger lacks async support, cannot display tasks or task info
//!     - [UI inspection tools are buggy](https://github.com/flutter/flutter-intellij/issues/4426), not fixed in 3 years
//!     - When your Dart code times out waiting for an HTTP request,
//!       [the request continues in the background](https://github.com/dart-lang/http/issues/424),
//!       wasting mobile device CPU & RAM, data transfer, battery, and server resources.
//!   </details>
//! - [Jasonelle](https://jasonelle.com)
//!   - Define app UI with JSON and JavaScript.
//!   - Incomplete documentation
//! - [Hyperview](https://github.com/instawork/hyperview)
//!   - Server-driven mobile apps
//!   - Define app UI in XML
//!   - iOS & Android (via `ReactNative`)
//! - [`DocUI`](https://nexusdev.tools)
//! - [Ionic](https://ionicframework.com)
//!   - Write app in HTML, CSS, and JavaScript.
//! - [Adaptive Cards](https://adaptivecards.io)
//!   - Libraries for rendering simple JSON UIs on iOS, Android, and Windows.
//!   - From Microsoft
//!
//! Companies using server-driven UI for popular apps:
//! - [A Deep Dive into Airbnb’s Server-Driven UI System](https://medium.com/airbnb-engineering/a-deep-dive-into-airbnbs-server-driven-ui-system-842244c5f5)
//! - [How we built Facebook Lite for every Android phone and network](https://engineering.fb.com/2016/03/09/android/how-we-built-facebook-lite-for-every-android-phone-and-network/)
//! - [Facebook Lite: Building for 2G Connections and Typical Devices (Video)](https://www.facebook.com/watch/?v=10153625275313553)
//!
//! # Changelog
//! - v0.1.0 - First published version
//!
//! # TO DO
//! - Clean shutdown.
//! - `SessionSet`: Remove disconnected clients from the set after a delay.
//! - Send keepalives.
//! - Session: Schedule only one worker at a time per session.
//! - Session: When not in an RPC, when building and an error or panic occurs, disconnect.
//! - Action to refresh an image
//! - Server to push refresh an image
#![forbid(unsafe_code)]

mod action;
mod action_builders;
mod alert_modal;
mod back_button;
mod button;
mod checkbox;
mod column;
mod context;
mod context_set;
mod detail_cell;
mod error;
mod h_alignment;
mod info_modal;
mod key_set;
mod list;
mod nav_page;
mod page_enum;
mod page_key;
mod plain_page;
mod question_modal;
mod random;
mod roster;
mod scroll;
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
    pub use crate::page_key::*;
}

pub mod builder {
    pub use crate::action_builders::*;
    pub use crate::alert_modal::*;
    pub use crate::back_button::*;
    pub use crate::button::*;
    pub use crate::checkbox::*;
    pub use crate::column::*;
    pub use crate::detail_cell::*;
    pub use crate::info_modal::*;
    pub use crate::list::*;
    pub use crate::nav_page::*;
    pub use crate::plain_page::*;
    pub use crate::question_modal::*;
    pub use crate::scroll::*;
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
