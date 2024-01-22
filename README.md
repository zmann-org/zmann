![zmann logo](.github/icons/logo-dark.png#gh-dark-mode-only)
![zmann logo](.github/icons/logo-light.png#gh-light-mode-only) 
**ZMANN**
=======
###### **Explore a range of instruments, designed to elevate your audio production experience.**
----
![GitHub License](https://img.shields.io/github/license/zmann-org/zmann?style=plastic)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/zmann-org/zmann/total?style=plastic)
![ZMANN Badge](https://img.shields.io/badge/zmann-monorepo-gold.svg?style=plastic&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iNjAwIiBoZWlnaHQ9IjYwMCIgdmlld0JveD0iMCAwIDYwMCA2MDAiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxwYXRoIGQ9Ik0zMjIuMDMgNkwwLjgwMTA2NiA2LjAwMDAxTDAgNi43OTcwMUwxNTUuNDA3IDE2MS40MThIMzMwLjA0TDUxLjI2ODQgNDM4Ljc3OUwxNTUuNDA3IDU0Mi4zOTFDMTkyLjI1NiA1NzkuMDU0IDI0My41MjUgNTk0LjE5NyAyODcuNTg0IDU5NC4xOTdINjAwTDQ0My43OTIgNDM4Ljc3OUgyNjkuMTU5TDU0OC43MzIgMTYwLjYyMUw0NzAuMjI3IDgyLjUxMzZDNDA2LjE0MiAxOC43NTIyIDM0Mi4wNTYgNS45OTk5NiAzMjIuMDMgNloiIGZpbGw9IndoaXRlIi8+Cjwvc3ZnPgo=)
![Static Badge](https://img.shields.io/badge/VST3-C90827?style=plastic&logo=steinberg)

## Plugins
As defined in [bundler.toml](./bundler.toml), the following plugins are included in this repository:

|Product|Build|Pluginval||
|---|---|---|---|
|[Toybox C1200 (Alpha)](./crates/toybox_c1200/)|![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/zmann-org/zmann/monorepo.yml?style=plastic)|![pluginval](https://img.shields.io/badge/pluginval-passing-green.svg?style=plastic)|[Download](https://github.com/zmann-org/zmann/releases)|
|Bilhorn Reed|   |   |   |
|Mellotron|   |   |   |
|   |   |   |   |

for more information on each plugin, please refer to their respective product pages on the zmann website.

## Installation
> [!TIP]
> You can make a `ZMANN` folder inside of your VST3 folder to keep your ZMANN plugins all in one place.

After downloading the latest release, extract the contents of the zip file to your VST3 folder. <!-- If you don't know where your VST3 folder is, -->

## Table of Contents
- [Plugins](#plugins)
- [Installation](#installation)
- [Screenshots](#screenshots)
- [Documentation](#documentation)
- [Building](#building)
  - [Prerequisites](#prerequisites)
  - [Building the plugins](#building-the-plugins)
  - [Cross-compiling](#cross-compiling)
- [Acknowledgments](#acknowledgments)

## Building
> [!WARNING]  
> As of writing, only Windows is supported for building. Linux cross-compilation might work, but is not stable at this time. [Read more about cross-compilation](#cross-compiling).
### Prerequisites
- [Moonrepo](https://moonrepo.dev/docs/install#windows)
- [Node.js 18.17](https://nodejs.org/en/) or later
- [rustup](https://www.rust-lang.org/tools/install)

ZMANN uses Moonrepo as its build tooling, make sure to install it before continuing.
Once installed, make sure that rustup is installed with **msvc** if building on Windows:
```bash
$ rustup default stable-x86_64-pc-windows-msvc
```

### Building the plugins
For each plugin in the repository, you can build manually by running the following command:
```bash
$ moon toybox_c1200:build
```

### Cross-Compiling
#### Debian/Ubuntu
Make sure to install the following package and toolchain:
```sh
$ sudo apt install build-essential
$ rustup target add x86_64-pc-windows-gnu
```
After installing the toolchain, modify [.cargo/config.toml](.cargo/config.toml) to use the `x86_64-pc-windows-gnu` toolchain:
```toml
[alias]
xtask = "run --package xtask --target x86_64-pc-windows-gnu --release --"
```
Finally, you can build the plugins by running the following command:
```bash
$ moon plugin_name:build
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

----
***VST is a registered trademark of Steinberg Media Technologies GmbH. ZMANN is not affilitated with Steinberg.***
