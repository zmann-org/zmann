![zmann logo](.github/icons/logo-dark.png#gh-dark-mode-only)
![zmann logo](.github/icons/logo-light.png#gh-light-mode-only) 
**ZMANN**
=======
###### **Explore a range of instruments, designed to elevate your audio production experience.**
----
![ZMANN Badge](https://img.shields.io/badge/zmann%20product-000.svg?style=for-the-badge&labelColor=000&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iNjAwIiBoZWlnaHQ9IjYwMCIgdmlld0JveD0iMCAwIDYwMCA2MDAiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxwYXRoIGQ9Ik0zMjIuMDMgNkwwLjgwMTA2NiA2LjAwMDAxTDAgNi43OTcwMUwxNTUuNDA3IDE2MS40MThIMzMwLjA0TDUxLjI2ODQgNDM4Ljc3OUwxNTUuNDA3IDU0Mi4zOTFDMTkyLjI1NiA1NzkuMDU0IDI0My41MjUgNTk0LjE5NyAyODcuNTg0IDU5NC4xOTdINjAwTDQ0My43OTIgNDM4Ljc3OUgyNjkuMTU5TDU0OC43MzIgMTYwLjYyMUw0NzAuMjI3IDgyLjUxMzZDNDA2LjE0MiAxOC43NTIyIDM0Mi4wNTYgNS45OTk5NiAzMjIuMDMgNloiIGZpbGw9IndoaXRlIi8+Cjwvc3ZnPgo=)
![GitHub License](https://img.shields.io/github/license/zmann-org/zmann?style=for-the-badge&labelColor=000)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/zmann-org/zmann/total?style=for-the-badge&labelColor=000)
![Static Badge](https://img.shields.io/badge/VST3-C90827?style=for-the-badge&logo=steinberg&labelColor=000)

## Plugins
As defined in [bundler.toml](./bundler.toml), the following plugins are included in this repository:

|Product|Build|Pluginval||
|---|---|---|---|
|[Toybox C1200](./crates/toybox_c1200/)|![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/zmann-org/zmann/monorepo.yml?style=for-the-badge&labelColor=000)|![pluginval](https://img.shields.io/badge/pluginval-passing-green.svg?style=for-the-badge&labelColor=000)|[Download](https://github.com/zmann-org/zmann/releases)|
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
Plugins are required to be built separately from each other. Run the *moon* command with the [plugin name](#plugins) and `:build` at the end to target a release build. For example:
```bash
$ moon toybox_c1200:build
```
Moonrepo will now build the `toybox_c1200_ui` before building the plugin itself. After successfully building, each plugin will make their own *.vst3* bundle inside of the `/target/bundled/` folder. 

Optionally by running `./scripts/link-bundled-to-dev.cmd` on Windows, a simlink will be created from the locally generated artifacts folder to the standard vst3 folder to streamline development.

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
***VST® is a trademark of Steinberg Media Technologies GmbH, registered in Europe and other countries. Other company names, product names and logos are the trademarks or registered trademarks of their respective owners.***
