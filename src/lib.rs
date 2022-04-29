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
pub mod context;
pub mod context_set;
pub mod error;
pub mod key_set;
pub mod pages;
pub mod random;
pub mod roster;
pub mod session;
pub mod session_cookie;
pub mod session_id;
pub mod session_set;
pub mod widget_list;
pub mod widgets;
