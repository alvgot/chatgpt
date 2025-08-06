# IOU App and smol-iroh

This workspace contains a prototype `smol`-based peer-to-peer IOU network and a minimal Android demo application.

## Prerequisites

1. Install [Rust](https://rustup.rs/) and ensure `cargo` is on your `PATH`.
2. Install the Android SDK and NDK (via Android Studio or the command-line tools).
3. Set the environment variables so build tools can locate the SDK and NDK:
   ```bash
   export ANDROID_SDK_ROOT=/path/to/android/sdk
   export ANDROID_NDK_HOME="$ANDROID_SDK_ROOT/ndk/<version>"
   ```
4. Add the Android target for Rust cross-compilation:
   ```bash
   rustup target add aarch64-linux-android
   ```
5. Install the `cargo-apk` subcommand used to package Rust projects as APKs:
   ```bash
   cargo install cargo-apk
   ```

## Building the Android APK

1. Enter the Android demo crate:
   ```bash
   cd iou-app
   ```
2. Build a release APK for 64-bit ARM devices:
   ```bash
   cargo apk build --release --target aarch64-linux-android
   ```
   The resulting file will be at `target/aarch64-linux-android/release/apk/iou-app.apk`.
3. (Optional) Install the APK on a connected device or emulator:
   ```bash
   adb install -r target/aarch64-linux-android/release/apk/iou-app.apk
   ```

## Testing

Run tests for the workspace crates:
```bash
cargo test
```

## Notes

This project is a proof of concept and uses placeholder cryptography and networking logic. Do not use in production.
