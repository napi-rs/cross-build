# cross build multi-platform addons on a single system

NAPI-RS is aiming to provide a complete solution for building Node.js native addons, especially for enterprise users.

For the open-source projects on GitHub, NAPI-RS provided out-of-box CI/CD support for building and publishing prebuilt binaries for Windows, Linux macOS and FreeBSD with NAPI-RS v2. There are also many users who are using self-hosted CI and want to build native addons for multiple platforms on a single system (mostly Linux). NAPI-RS v2 provides a solution for this use case. 

This project is a demo for those users who want to build multi-platform addons on a single system. The CI of this project builds NAPI-RS packages on a GitHub Linux CI, and produces addons for the following platform:

* Windows x64
* Windows x86
* Windows arm64
* macOS x64
* macOS arm64
* Linux x64 gnu (`glibc` 2.17)
* Linux x64 musl
* Linux arm64 gnu (`glibc` 2.17)
* Linux arm64 musl
* Linux armv7 gnueabihf
* Android arm64
* Android armv7

If you are building NAPI-RS packages with self-hosted Linux CI, you should install the following toolchain to build the addons for the above platforms:

* `Rust` toolchain, including `Cargo`, `rustup` and your target rust std. eg: `rustup target add aarch64-apple-darwin`.
- `llvm`. The latest *qualification* version is recommended.
- `cargo-xwin` if you want to build for Windows on the non Windows system. `cargo install cargo-xwin`.