Maggie
======
[![crates.io version](https://img.shields.io/crates/v/maggie.svg)](https://crates.io/crates/maggie)
[![license: Apache 2.0](https://raw.githubusercontent.com/mleonhard/maggie-rs/main/license-apache-2.0.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![unsafe forbidden](https://raw.githubusercontent.com/mleonhard/maggie-rs/main/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![pipeline status](https://github.com/mleonhard/maggie-rs/workflows/CI/badge.svg)](https://github.com/mleonhard/maggie-rs/actions)

An app server library in Rust.

Use a Maggie client to access the app:
- `maggie-ios`
- `maggie-android`

# Features
- `forbid(unsafe_code)`
- Define your app's UI and RPCs in threaded Rust.
- Uses [`beatrice`](https://github.com/mleonhard/beatrice-rs)
  http server for good performance under load. - TODO: Make a benchmark.
- No macros or complicated type params
- Good test coverage (??%) - TODO: Update.

# Limitations
- New, not proven in production.

# Examples
Complete examples: [`examples/`](examples/).

Simple example:
```rust
```
# Cargo Geiger Safety Report
```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒  maggie 0.1.0
0/0        0/0          0/0    0/0     0/0      🔒  ├── beatrice 0.3.0
0/0        4/4          0/0    0/0     2/2      ☢️  │   ├── async-fs 1.5.0
0/0        51/51        14/14  0/0     0/0      ☢️  │   │   ├── async-lock 2.5.0
0/0        106/116      4/8    0/0     0/0      ☢️  │   │   │   └── event-listener 2.5.2
0/0        28/28        4/4    0/0     0/0      ☢️  │   │   ├── blocking 1.2.0
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   ├── async-channel 1.6.1
0/0        155/155      2/2    0/0     1/1      ☢️  │   │   │   │   ├── concurrent-queue 1.2.2
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   │   │   └── cache-padded 1.2.0
0/0        106/116      4/8    0/0     0/0      ☢️  │   │   │   │   ├── event-listener 2.5.2
0/0        30/30        2/2    0/0     0/0      ☢️  │   │   │   │   └── futures-core 0.3.21
1/1        802/802      4/4    0/0     10/10    ☢️  │   │   │   ├── async-task 4.2.0
0/0        26/26        2/2    0/0     0/0      ☢️  │   │   │   ├── atomic-waker 1.0.0
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   ├── fastrand 1.7.0
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   ├── futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   │   ├── fastrand 1.7.0
0/0        30/30        2/2    0/0     0/0      ☢️  │   │   │   │   ├── futures-core 0.3.21
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   │   ├── futures-io 0.3.21
36/37      2067/2140    0/0    0/0     16/16    ☢️  │   │   │   │   ├── memchr 2.4.1
1/20       10/353       0/2    0/0     5/38     ☢️  │   │   │   │   │   └── libc 0.2.121
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   │   ├── parking 2.0.0
0/0        11/165       0/0    0/0     2/2      ☢️  │   │   │   │   ├── pin-project-lite 0.2.8
0/0        21/21        0/0    0/0     4/4      ☢️  │   │   │   │   └── waker-fn 1.1.0
1/1        74/93        4/6    0/0     2/3      ☢️  │   │   │   └── once_cell 1.10.0
0/0        0/0          0/0    0/0     0/0      ❓  │   │   └── futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── async-net 1.6.1
0/0        22/22        0/0    0/0     0/0      ☢️  │   │   ├── async-io 1.6.0
0/0        155/155      2/2    0/0     1/1      ☢️  │   │   │   ├── concurrent-queue 1.2.2
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   ├── futures-lite 1.12.0
1/20       10/353       0/2    0/0     5/38     ☢️  │   │   │   ├── libc 0.2.121
1/1        16/16        1/1    0/0     0/0      ☢️  │   │   │   ├── log 0.4.16
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   │   ├── cfg-if 1.0.0
0/0        5/5          0/0    0/0     0/0      ☢️  │   │   │   │   └── serde 1.0.136
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   │       └── serde_derive 1.0.136
0/0        12/12        0/0    0/0     3/3      ☢️  │   │   │   │           ├── proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   │           │   └── unicode-xid 0.2.2
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   │           ├── quote 1.0.16
0/0        12/12        0/0    0/0     3/3      ☢️  │   │   │   │           │   └── proc-macro2 1.0.36
0/0        47/47        3/3    0/0     2/2      ☢️  │   │   │   │           └── syn 1.0.89
0/0        12/12        0/0    0/0     3/3      ☢️  │   │   │   │               ├── proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   │               ├── quote 1.0.16
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   │               └── unicode-xid 0.2.2
1/1        74/93        4/6    0/0     2/3      ☢️  │   │   │   ├── once_cell 1.10.0
0/0        0/0          0/0    0/0     0/0      🔒  │   │   │   ├── parking 2.0.0
0/0        0/9          1/6    0/0     0/0      ☢️  │   │   │   ├── polling 2.2.0
0/0        0/0          0/0    0/0     0/0      ❓  │   │   │   │   ├── cfg-if 1.0.0
1/20       10/353       0/2    0/0     5/38     ☢️  │   │   │   │   ├── libc 0.2.121
1/1        16/16        1/1    0/0     0/0      ☢️  │   │   │   │   └── log 0.4.16
0/0        25/25        0/0    0/0     3/3      ☢️  │   │   │   ├── slab 0.4.5
0/0        5/5          0/0    0/0     0/0      ☢️  │   │   │   │   └── serde 1.0.136
3/6        528/641      2/4    0/0     3/4      ☢️  │   │   │   ├── socket2 0.4.4
1/20       10/353       0/2    0/0     5/38     ☢️  │   │   │   │   └── libc 0.2.121
0/0        21/21        0/0    0/0     4/4      ☢️  │   │   │   └── waker-fn 1.1.0
0/0        28/28        4/4    0/0     0/0      ☢️  │   │   ├── blocking 1.2.0
0/0        0/0          0/0    0/0     0/0      ❓  │   │   └── futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── fixed-buffer 0.5.0
0/0        0/0          0/0    0/0     0/0      ❓  │   │   └── futures-io 0.3.21
0/0        0/0          0/0    0/0     0/0      ❓  │   ├── futures-io 0.3.21
0/0        0/0          0/0    0/0     0/0      ❓  │   ├── futures-lite 1.12.0
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── permit 0.1.4
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── safe-regex 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒  │   │   └── safe-regex-macro 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒  │   │       ├── safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      🔒  │   │       │   └── unicode-xid 0.2.2
0/0        0/0          0/0    0/0     0/0      🔒  │   │       └── safe-regex-compiler 0.2.5
0/0        0/0          0/0    0/0     0/0      🔒  │   │           ├── safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      🔒  │   │           └── safe-quote 1.0.15
0/0        0/0          0/0    0/0     0/0      🔒  │   │               └── safe-proc-macro2 1.0.36
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── safina-executor 0.3.3
0/0        0/0          0/0    0/0     0/0      🔒  │   │   ├── safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒  │   │   └── safina-threadpool 0.2.3
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── safina-sync 0.2.4
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── safina-timer 0.1.11
1/1        74/93        4/6    0/0     2/3      ☢️  │   │   └── once_cell 1.10.0
0/0        5/5          0/0    0/0     0/0      ☢️  │   ├── serde 1.0.136
0/0        4/7          0/0    0/0     0/0      ☢️  │   ├── serde_json 1.0.79
0/0        7/7          0/0    0/0     0/0      ☢️  │   │   ├── itoa 1.0.1
7/9        587/723      0/0    0/0     2/2      ☢️  │   │   ├── ryu 1.0.9
0/0        5/5          0/0    0/0     0/0      ☢️  │   │   └── serde 1.0.136
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── temp-dir 0.1.11
0/0        0/0          0/0    0/0     0/0      🔒  │   ├── temp-file 0.1.7
0/0        0/0          0/0    0/0     0/0      ❓  │   └── url 2.2.2
0/0        2/2          0/0    0/0     0/0      ☢️  │       ├── form_urlencoded 1.0.1
0/0        0/0          0/0    0/0     0/0      ❓  │       │   ├── matches 0.1.9
0/0        3/3          0/0    0/0     0/0      ☢️  │       │   └── percent-encoding 2.1.0
0/0        0/0          0/0    0/0     0/0      ❓  │       ├── idna 0.2.3
0/0        0/0          0/0    0/0     0/0      ❓  │       │   ├── matches 0.1.9
0/0        0/0          0/0    0/0     0/0      🔒  │       │   ├── unicode-bidi 0.3.7
0/0        5/5          0/0    0/0     0/0      ☢️  │       │   │   └── serde 1.0.136
0/0        20/20        0/0    0/0     0/0      ☢️  │       │   └── unicode-normalization 0.1.19
0/0        0/0          0/0    0/0     0/0      🔒  │       │       └── tinyvec 1.5.1
0/0        5/5          0/0    0/0     0/0      ☢️  │       │           ├── serde 1.0.136
0/0        0/0          0/0    0/0     0/0      ❓  │       │           └── tinyvec_macros 0.1.0
0/0        0/0          0/0    0/0     0/0      ❓  │       ├── matches 0.1.9
0/0        3/3          0/0    0/0     0/0      ☢️  │       ├── percent-encoding 2.1.0
0/0        5/5          0/0    0/0     0/0      ☢️  │       └── serde 1.0.136
0/0        7/20         0/0    0/0     0/0      ☢️  ├── nanorand 0.7.0
1/1        74/93        4/6    0/0     2/3      ☢️  ├── once_cell 1.10.0
0/0        5/5          0/0    0/0     0/0      ☢️  ├── serde 1.0.136
0/0        4/7          0/0    0/0     0/0      ☢️  └── serde_json 1.0.79

50/75      4670/5543    43/58  0/0     55/90  

```
# Alternatives

- [React Native](https://reactnative.dev/)
  - Very popular
  - iOS & Android
  - Write apps in TypeScript or JavaScript
  - Immediately deploy new versions
  - Develop on Linux, macOS, & Windows
  - Toolchain is diverse and not cohesive.
- [Xcode](https://developer.apple.com/xcode/)
  - Very popular
  - iOS
  - Write apps in Swift or Objective-C
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on macOS only
  - [Docs are not searchable](https://github.com/apple/swift-org-website/issues/24)
- [Android Studio](https://developer.android.com/studio/)
  - Very popular
  - Android
  - Write apps in Kotlin or Java
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on Linux, macOS, Windows, & Chrome OS
- [Xamarin](https://dotnet.microsoft.com/en-us/apps/xamarin)
  - iOS & Android
  - Write apps in C# and other .Net language
  - Updates take multiple days to deploy.
  - Users choose when to update.
    Server must support multiple app versions or demand that users update.
  - Develop on Linux, macOS, & Windows
- [Flutter](https://flutter.dev/)
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
    - Debugger lacks async support
    - [UI inspection tools are buggy](https://github.com/flutter/flutter-intellij/issues/4426), not fixed in 3 years
    - When your Dart code times out waiting for an HTTP request,
      [the request continues in the background](https://github.com/dart-lang/http/issues/424),
      wasting mobile device CPU & RAM, data transfer, battery, and server resources.

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

License: MIT OR Apache-2.0
