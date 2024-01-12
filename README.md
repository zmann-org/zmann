# ZMANN
Collection of VST's by ZMANN built using [`nih-plug`](https://github.com/robbert-vdh/nih-plug).

## Effects

- [x] gain
- [x] chorus
- [x] vibrato
- [x] reverb (delay?)
- [x] output

## Building

After installing [Rust](https://rustup.rs/), you can compile Toybox C1200 as follows:

#### Windows

```sh
./scripts/download-vst.bat

cargo xtask bundle toybox_c1200 --release
```

#### Debian/Ubuntu

Make sure that the required build tools are installed:

```sh
rustup target add x86_64-pc-windows-gnu
sudo apt install build-essential
```

Before the final build step, you need to get the `binv3` files:

1. **Download Precompiled Files:** You can download the precompiled `binv3` files from Zmann's CDN. To do this, run the following script:

    ```sh
    ./scripts/download-binv3.bat
    ```

2. **Build Yourself:** The option of building them yourself is currently not supported.

Then finally to build the plugin run:

```sh
cargo xtask bundle toybox_c1200 --target x86_64-pc-windows-gnu
```

## Acknowledgments

- **wry** (Modified)
  - Original Library: [wry](https://github.com/tauri-apps/wry)
  - License: Apache-2.0

- **nih-plug-webview** (Modified)
  - Original Library: [nih-plug-webview](https://github.com/maxjvh/nih-plug-webview)
  - License: ISC

- **fx** (Modified)
  - Original Library: [renzol2/fx](https://github.com/renzol2/fx)
  - License: GPLv3

- **nih-plug** (Integrated)
  - Original Library: [nih-plug](https://github.com/robbert-vdh/nih-plug)
    - The framework is licensed under the ISC license.
    - The VST3 bindings used by `nih_export_vst3!()` are licensed under the GPLv3 license. This implies that unless you replace these bindings with your own bindings made from scratch, any VST3 plugins built in this repository need to comply with the terms of the GPLv3 license.

The code in this repository is licensed under the GNU General Public License v3.0 or later. You can find a copy of the license in the [LICENSE](./LICENSE) file.

The samples included in this repository are owned by their respective owners. Please refer to the individual sample files for their specific licensing information.
