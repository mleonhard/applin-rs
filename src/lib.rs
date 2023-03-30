//! Applin™
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
//! - applin-android (Coming soon!)
//!
//! # Features
//! - `forbid(unsafe_code)`
//! - Define your app's UI and RPCs in threaded Rust.
//! - Uses [Servlin](https://crates.io/crates/servlin)
//!   http server for good performance under load. - TODO: Make a benchmark.
//! - No macros or complicated type params
//! - Good test coverage (??%) - TODO: Update.
//!
//! # Limitations
//! - New, not proven in production.
//!
//! # Examples
//! Complete examples: [`examples/`](https://github.com/mleonhard/applin-rs/tree/main/examples).
//!
//! Minimal example:
//! ```no_run
//! use applin::data::Rebuilder;
//! use applin::session::{PageMap, SessionSet};
//! use applin::widget::{NavPage, Text};
//! use servlin::reexport::{safina_executor, safina_timer};
//! use servlin::{socket_addr_127_0_0_1, HttpServerBuilder, Request};
//! use std::sync::Arc;
//!
//! safina_timer::start_timer_thread();
//! let executor = safina_executor::Executor::default();
//! let sessions: Arc<SessionSet<()>> = Arc::new(SessionSet::new(&executor));
//! let page_map_fn = move |_rebuilder: Rebuilder<()>| {
//!     Ok(PageMap::new().with_static_page(
//!         "/",
//!         NavPage::new("Minimal Example", Text::new("Hello")).with_poll(10),
//!     ))
//! };
//! let session_state_fn = move || ();
//! let req_handler =
//!     move |req: Request| match sessions.get_or_new(&req, page_map_fn, session_state_fn) {
//!         Ok(session) => session.poll().unwrap_or_else(|response| response),
//!         Err(response) => response,
//!     };
//! executor
//!     .block_on(
//!         HttpServerBuilder::new()
//!             .listen_addr(socket_addr_127_0_0_1(8000))
//!             .spawn_and_join(req_handler),
//!     )
//!     .unwrap();
//! ```
//! ![Screenshot of minimal.rs](examples/minimal-screenshot.png "Screenshot of minimal.rs")
//!
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
//! - [ZK](https://www.zkoss.org/)
//!   - Define app UI and behavior with XML and server-side Java.
//!   - Commercial product with free Community Edition.
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
//! - [Azilen SDUI](https://www.azilen.com/blog/server-driven-user-interface-sdui-framework-ios-applications)
//!   - Proprietary server-driven UI framework offered by a consulting company, since 2016
//! - [Unflow](https://www.unflow.com)
//!   - A hosted service that lets you design and update app pages without changing code.
//!
//! Companies using server-driven UI for popular apps:
//! - Airbnb: [A Deep Dive into Airbnb’s Server-Driven UI System](https://medium.com/airbnb-engineering/a-deep-dive-into-airbnbs-server-driven-ui-system-842244c5f5)
//! - Facebook: [How we built Facebook Lite for every Android phone and network](https://engineering.fb.com/2016/03/09/android/how-we-built-facebook-lite-for-every-android-phone-and-network/)
//! - Facebook: [Facebook Lite: Building for 2G Connections and Typical Devices (Video)](https://www.facebook.com/watch/?v=10153625275313553)
//! - Doordash: [Improving Development Velocity with Generic, Server-Driven UI Components](https://doordash.engineering/2021/08/24/improving-development-velocity-with-generic-server-driven-ui-components/)
//! - Spotify: [Hub Framework - Spotify’s component-driven UI framework for iOS](https://github.com/spotify/HubFramework)
//! - Instacart: [Building Instacart’s view model API](https://tech.instacart.com/building-instacarts-view-model-api-part-1-why-view-model-4362f64ffd2a)
//!
//! [Introduction to Server Driven UI in iOS, Swift & SwiftUI - Udemy course](https://www.udemy.com/course/introduction-to-server-driven-ui-in-ios-swift-swiftui/)
//!
//! [Server driven UI and Apple review guidelines. - r/iOSProgramming - Reddit](https://old.reddit.com/r/iOSProgramming/comments/11m5aq1/server_driven_ui_and_apple_review_guidelines/)
//!
//! # Trademark
//! "Applin™" is a trademark of Leonhard LLC.
//! You may not use the trademark without written authorization from Leonhard LLC.
//!
//! Apple released their iOS `UIKit` library in 2008.
//! Then in 2013, another group released a web framework also called `UIKit`.
//! Because the two libraries have the same name,
//! search results contain info for both libraries.
//! iOS developers searching for info must
//! waste time reading pages about the web framework.
//! This is a serious usability problem.
//! I plan to use trademark law to prevent similar difficulties for Applin users.
//!
//! I intend to grant authorization to use the "Applin" name
//! to high-quality compatible libraries and tools.
//! For example, if you wish to write applin-go or applin-rails, please contact me.
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
//! - Fail build on key collision, for large projects.
//! - Allow customizing widget style.  See https://github.com/Lona/Lona .
//! - Start Android implementation
//!    - https://github.com/flipkart-incubator/proteus
#![forbid(unsafe_code)]

pub mod action;
pub mod data;
pub mod error;
pub mod internal;
pub mod session;
pub mod widget;

pub mod reexports {
    pub use serde_json;
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    *t == Default::default()
}
