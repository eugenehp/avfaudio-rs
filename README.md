# avfaudio

A rust version of setting up AVFAudio and AVAudioSession.

Based on https://github.com/eugenehp/avfaudio-sys

> [!WARNING]  
> Work in progress.
> This library implements only `AVAudioSession` category setting for now.
> For full bindings check [avfaudio2-sys](https://github.com/eugenehp/avfaudio-sys).

## Development

### Build

Build on your host system (mac):
```shell
cargo build
```

Or specify target like `aarch64-apple-visionos`, rest of the parameters are needed for the `nightly` rust toolchain, since visionOS is a Tier 3 target.

```shell
cargo +nightly build -Zbuild-std --target aarch64-apple-visionos
```

### Run example

```shell
cargo run --example session
```

## Authors

Copyright (c) 2024 Eugene Hauptmann

## License

[MIT](/LICENSE)