Applinβ’
======
[![crates.io version](https://img.shields.io/crates/v/applin.svg)](https://crates.io/crates/applin)
[![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/applin-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/applin-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://github.com/mleonhard/applin-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/applin-rs/actions)

An app server library in Rust.

Use an Applin client to access the app:
- [applin-ios](https://github.com/mleonhard/applin-ios)
- [applin-android](https://github.com/mleonhard/applin-android)

# Features
- `forbid(unsafe_code)`
- Define your app's UI and RPCs in threaded Rust.
- Uses [Servlin](https://crates.io/crates/servlin)
  http server for good performance under load. - TODO: Make a benchmark.
- No macros or complicated type params
- Good test coverage (??%) - TODO: Update.

# Limitations
- New, not proven in production.

# Examples
Complete examples: [`examples/`](https://github.com/mleonhard/applin-rs/tree/main/examples).

Simple example:
```rust
// TODO: Add this.
```
# Cargo Geiger Safety Report
```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    π  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    β  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    β’οΈ  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      π  applin 0.1.0
0/0        7/20         0/0    0/0     0/0      β’οΈ  βββ nanorand 0.7.0
1/1        74/93        4/6    0/0     2/3      β’οΈ  βββ once_cell 1.10.0
0/0        5/5          0/0    0/0     0/0      β’οΈ  βββ serde 1.0.136
0/0        0/0          0/0    0/0     0/0      β  β   βββ serde_derive 1.0.136
0/0        12/12        0/0    0/0     3/3      β’οΈ  β       βββ proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      π  β       β   βββ unicode-xid 0.2.2
0/0        0/0          0/0    0/0     0/0      β  β       βββ quote 1.0.16
0/0        12/12        0/0    0/0     3/3      β’οΈ  β       β   βββ proc-macro2 1.0.36
0/0        47/47        3/3    0/0     2/2      β’οΈ  β       βββ syn 1.0.89
0/0        12/12        0/0    0/0     3/3      β’οΈ  β           βββ proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      β  β           βββ quote 1.0.16
0/0        0/0          0/0    0/0     0/0      π  β           βββ unicode-xid 0.2.2
0/0        4/7          0/0    0/0     0/0      β’οΈ  βββ serde_json 1.0.79
0/0        7/7          0/0    0/0     0/0      β’οΈ  β   βββ itoa 1.0.1
7/9        587/723      0/0    0/0     2/2      β’οΈ  β   βββ ryu 1.0.9
0/0        5/5          0/0    0/0     0/0      β’οΈ  β   βββ serde 1.0.136
0/0        0/0          0/0    0/0     0/0      π  βββ servlin 0.1.1
0/0        4/4          0/0    0/0     2/2      β’οΈ      βββ async-fs 1.5.0
0/0        51/51        14/14  0/0     0/0      β’οΈ      β   βββ async-lock 2.5.0
0/0        106/116      4/8    0/0     0/0      β’οΈ      β   β   βββ event-listener 2.5.2
0/0        28/28        4/4    0/0     0/0      β’οΈ      β   βββ blocking 1.2.0
0/0        0/0          0/0    0/0     0/0      π      β   β   βββ async-channel 1.6.1
0/0        155/155      2/2    0/0     1/1      β’οΈ      β   β   β   βββ concurrent-queue 1.2.2
0/0        0/0          0/0    0/0     0/0      π      β   β   β   β   βββ cache-padded 1.2.0
0/0        106/116      4/8    0/0     0/0      β’οΈ      β   β   β   βββ event-listener 2.5.2
0/0        30/30        2/2    0/0     0/0      β’οΈ      β   β   β   βββ futures-core 0.3.21
1/1        802/802      4/4    0/0     10/10    β’οΈ      β   β   βββ async-task 4.2.0
0/0        26/26        2/2    0/0     0/0      β’οΈ      β   β   βββ atomic-waker 1.0.0
0/0        0/0          0/0    0/0     0/0      π      β   β   βββ fastrand 1.7.0
0/0        0/0          0/0    0/0     0/0      β      β   β   βββ futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      π      β   β   β   βββ fastrand 1.7.0
0/0        30/30        2/2    0/0     0/0      β’οΈ      β   β   β   βββ futures-core 0.3.21
0/0        0/0          0/0    0/0     0/0      β      β   β   β   βββ futures-io 0.3.21
36/37      2067/2140    0/0    0/0     16/16    β’οΈ      β   β   β   βββ memchr 2.4.1
1/20       10/353       0/2    0/0     5/38     β’οΈ      β   β   β   β   βββ libc 0.2.121
0/0        0/0          0/0    0/0     0/0      π      β   β   β   βββ parking 2.0.0
0/0        11/165       0/0    0/0     2/2      β’οΈ      β   β   β   βββ pin-project-lite 0.2.8
0/0        21/21        0/0    0/0     4/4      β’οΈ      β   β   β   βββ waker-fn 1.1.0
1/1        74/93        4/6    0/0     2/3      β’οΈ      β   β   βββ once_cell 1.10.0
0/0        0/0          0/0    0/0     0/0      β      β   βββ futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      π      βββ async-net 1.6.1
0/0        22/22        0/0    0/0     0/0      β’οΈ      β   βββ async-io 1.6.0
0/0        155/155      2/2    0/0     1/1      β’οΈ      β   β   βββ concurrent-queue 1.2.2
0/0        0/0          0/0    0/0     0/0      β      β   β   βββ futures-lite 1.12.0
1/20       10/353       0/2    0/0     5/38     β’οΈ      β   β   βββ libc 0.2.121
1/1        16/16        1/1    0/0     0/0      β’οΈ      β   β   βββ log 0.4.16
0/0        0/0          0/0    0/0     0/0      β      β   β   β   βββ cfg-if 1.0.0
0/0        5/5          0/0    0/0     0/0      β’οΈ      β   β   β   βββ serde 1.0.136
1/1        74/93        4/6    0/0     2/3      β’οΈ      β   β   βββ once_cell 1.10.0
0/0        0/0          0/0    0/0     0/0      π      β   β   βββ parking 2.0.0
0/0        0/9          1/6    0/0     0/0      β’οΈ      β   β   βββ polling 2.2.0
0/0        0/0          0/0    0/0     0/0      β      β   β   β   βββ cfg-if 1.0.0
1/20       10/353       0/2    0/0     5/38     β’οΈ      β   β   β   βββ libc 0.2.121
1/1        16/16        1/1    0/0     0/0      β’οΈ      β   β   β   βββ log 0.4.16
0/0        25/25        0/0    0/0     3/3      β’οΈ      β   β   βββ slab 0.4.5
0/0        5/5          0/0    0/0     0/0      β’οΈ      β   β   β   βββ serde 1.0.136
3/6        528/641      2/4    0/0     3/4      β’οΈ      β   β   βββ socket2 0.4.4
1/20       10/353       0/2    0/0     5/38     β’οΈ      β   β   β   βββ libc 0.2.121
0/0        21/21        0/0    0/0     4/4      β’οΈ      β   β   βββ waker-fn 1.1.0
0/0        28/28        4/4    0/0     0/0      β’οΈ      β   βββ blocking 1.2.0
0/0        0/0          0/0    0/0     0/0      β      β   βββ futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      π      βββ fixed-buffer 0.5.0
0/0        0/0          0/0    0/0     0/0      β      β   βββ futures-io 0.3.21
0/0        0/0          0/0    0/0     0/0      β      βββ futures-io 0.3.21
0/0        0/0          0/0    0/0     0/0      β      βββ futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      π      βββ permit 0.1.4
0/0        0/0          0/0    0/0     0/0      π      βββ safe-regex 0.2.5
0/0        0/0          0/0    0/0     0/0      π      β   βββ safe-regex-macro 0.2.5
0/0        0/0          0/0    0/0     0/0      π      β       βββ safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      π      β       β   βββ unicode-xid 0.2.2
0/0        0/0          0/0    0/0     0/0      π      β       βββ safe-regex-compiler 0.2.5
0/0        0/0          0/0    0/0     0/0      π      β           βββ safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      π      β           βββ safe-quote 1.0.15
0/0        0/0          0/0    0/0     0/0      π      β               βββ safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      π      βββ safina-executor 0.3.3
0/0        0/0          0/0    0/0     0/0      π      β   βββ safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      π      β   βββ safina-threadpool 0.2.3
0/0        0/0          0/0    0/0     0/0      π      βββ safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      π      βββ safina-timer 0.1.11
1/1        74/93        4/6    0/0     2/3      β’οΈ      β   βββ once_cell 1.10.0
0/0        5/5          0/0    0/0     0/0      β’οΈ      βββ serde 1.0.136
0/0        4/7          0/0    0/0     0/0      β’οΈ      βββ serde_json 1.0.79
0/0        0/0          0/0    0/0     0/0      π      βββ temp-dir 0.1.11
0/0        0/0          0/0    0/0     0/0      π      βββ temp-file 0.1.7
0/0        0/0          0/0    0/0     0/0      β      βββ url 2.2.2
0/0        2/2          0/0    0/0     0/0      β’οΈ          βββ form_urlencoded 1.0.1
0/0        0/0          0/0    0/0     0/0      β          β   βββ matches 0.1.9
0/0        3/3          0/0    0/0     0/0      β’οΈ          β   βββ percent-encoding 2.1.0
0/0        0/0          0/0    0/0     0/0      β          βββ idna 0.2.3
0/0        0/0          0/0    0/0     0/0      β          β   βββ matches 0.1.9
0/0        0/0          0/0    0/0     0/0      π          β   βββ unicode-bidi 0.3.7
0/0        5/5          0/0    0/0     0/0      β’οΈ          β   β   βββ serde 1.0.136
0/0        20/20        0/0    0/0     0/0      β’οΈ          β   βββ unicode-normalization 0.1.19
0/0        0/0          0/0    0/0     0/0      π          β       βββ tinyvec 1.5.1
0/0        5/5          0/0    0/0     0/0      β’οΈ          β           βββ serde 1.0.136
0/0        0/0          0/0    0/0     0/0      β          β           βββ tinyvec_macros 0.1.0
0/0        0/0          0/0    0/0     0/0      β          βββ matches 0.1.9
0/0        3/3          0/0    0/0     0/0      β’οΈ          βββ percent-encoding 2.1.0
0/0        5/5          0/0    0/0     0/0      β’οΈ          βββ serde 1.0.136

50/75      4670/5543    43/58  0/0     55/90  

```
# Alternatives

- [React Native](https://reactnative.dev/)
  <details><summary>Details</summary>

  - Very popular
  - iOS & Android
  - Write apps in TypeScript or JavaScript
  - Immediately deploy new versions
  - Develop on Linux, macOS, & Windows
  - Toolchain is diverse and not cohesive.
  </details>
- [Xcode](https://developer.apple.com/xcode/)
  <details><summary>Details</summary>

  - Very popular
  - iOS
  - Write apps in Swift or Objective-C
  - Use `UIKit` or `SwiftUI` libraries
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on macOS only
  - [Docs are not searchable](https://github.com/apple/swift-org-website/issues/24)
  </details>
- [Android Studio](https://developer.android.com/studio/)
  <details><summary>Details</summary>

  - Very popular
  - Android
  - Write apps in Kotlin or Java
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on Linux, macOS, Windows, & Chrome OS
  </details>
- [Xamarin](https://dotnet.microsoft.com/en-us/apps/xamarin)
  <details><summary>Details</summary>

  - iOS & Android
  - Write apps in C# and other .Net language
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on Linux, macOS, & Windows
  </details>
- [Flutter](https://flutter.dev/)
  <details><summary>Details</summary>

  - iOS & Android
  - Web version has unusable performance
  - Write apps in async Dart
  - Good package management system
  - Compiled code is very small
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on Linux, macOS, & Windows
  - Widgets look and behave similar to native widgets
  - Unmaintained - little progress on mobile features since 2019
    - iOS support is incomplete:
      no detail cell widget or check box (Flutter Team replied to my email and refused to add these),
      [broken dark mode support](https://github.com/flutter/flutter/issues/80860),
      [no scroll-to-dismiss keyboard behavior](https://github.com/flutter/flutter/issues/57609) which is used by most iOS apps,
      [no keyboard dismiss button](https://github.com/flutter/flutter/issues/45076) which is used by all the other iOS apps,
    - Documentation has large holes:
      [Navigator](https://github.com/flutter/flutter/issues/69315),
      [focus management](https://github.com/flutter/flutter/issues/45076),
      non-trivial examples, local data management
    - [Integration test support is broken & deprecated](https://github.com/flutter/flutter/issues?q=is%3Aissue+author%3Amleonhard+integration+test+).
    - Camera & photo module has a [show-stopping bug](https://github.com/flutter/flutter/issues/70751), ignored for 3 years
    - [No high-quality location module](https://github.com/flutter/flutter/issues/31453)
    - Debugger lacks async support, cannot display tasks or task info
    - [UI inspection tools are buggy](https://github.com/flutter/flutter-intellij/issues/4426), not fixed in 3 years
    - When your Dart code times out waiting for an HTTP request,
      [the request continues in the background](https://github.com/dart-lang/http/issues/424),
      wasting mobile device CPU & RAM, data transfer, battery, and server resources.
  </details>
- [ZK](https://www.zkoss.org/)
  - Define app UI and behavior with XML and server-side Java.
  - Commercial product with free Community Edition.
- [Jasonelle](https://jasonelle.com)
  - Define app UI with JSON and JavaScript.
  - Incomplete documentation
- [Hyperview](https://github.com/instawork/hyperview)
  - Server-driven mobile apps
  - Define app UI in XML
  - iOS & Android (via `ReactNative`)
- [`DocUI`](https://nexusdev.tools)
- [Ionic](https://ionicframework.com)
  - Write app in HTML, CSS, and JavaScript.
- [Adaptive Cards](https://adaptivecards.io)
  - Libraries for rendering simple JSON UIs on iOS, Android, and Windows.
  - From Microsoft

Companies using server-driven UI for popular apps:
- [A Deep Dive into Airbnbβs Server-Driven UI System](https://medium.com/airbnb-engineering/a-deep-dive-into-airbnbs-server-driven-ui-system-842244c5f5)
- [How we built Facebook Lite for every Android phone and network](https://engineering.fb.com/2016/03/09/android/how-we-built-facebook-lite-for-every-android-phone-and-network/)
- [Facebook Lite: Building for 2G Connections and Typical Devices (Video)](https://www.facebook.com/watch/?v=10153625275313553)

# Trademark
"Applinβ’" is a trademark of Leonhard LLC.
You may not use the trademark without written authorization from Leonhard LLC.

Apple released their iOS `UIKit` library in 2008.
Then in 2013, another group released a web framework also called `UIKit`.
Because the two libraries have the same name,
search results contain info for both libraries.
iOS developers searching for info must
waste time reading pages about the web framework.
This is a serious usability problem.
I plan to use trademark law to prevent similar difficulties for Applin users.

I intend to grant authorization to use the "Applin" name
to high-quality compatible libraries and tools.
For example, if you wish to write applin-go or applin-rails, please contact me.

# Changelog
- v0.1.0 - First published version

# TO DO
- Clean shutdown.
- `SessionSet`: Remove disconnected clients from the set after a delay.
- Send keepalives.
- Session: Schedule only one worker at a time per session.
- Session: When not in an RPC, when building and an error or panic occurs, disconnect.
- Action to refresh an image
- Server to push refresh an image
- Fail build on key collision, for large projects.

License: MIT OR Apache-2.0
