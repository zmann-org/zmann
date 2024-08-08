![zmann logo](.github/icons/logo-dark.png#gh-dark-mode-only)
![zmann logo](.github/icons/logo-light.png#gh-light-mode-only) 
**ZMANN**
=======
###### **Explore a range of instruments, designed to elevate your audio production experience.**
<h1 align="center">

![gh-banner](https://raw.githubusercontent.com/zmann-org/zmann/main/.github/marketing/header-transparent-crop.png)

</h1>

> [!IMPORTANT]  
> We're working on a restructure of the repository, and planning on switching to a different GUI for the plugins. Read more at [#477](https://github.com/zmann-org/zmann/issues/477)

![ZMANN Badge](https://img.shields.io/badge/zmann%20products-000.svg?style=for-the-badge&labelColor=000&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iNjAwIiBoZWlnaHQ9IjYwMCIgdmlld0JveD0iMCAwIDYwMCA2MDAiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxwYXRoIGQ9Ik0zMjIuMDMgNkwwLjgwMTA2NiA2LjAwMDAxTDAgNi43OTcwMUwxNTUuNDA3IDE2MS40MThIMzMwLjA0TDUxLjI2ODQgNDM4Ljc3OUwxNTUuNDA3IDU0Mi4zOTFDMTkyLjI1NiA1NzkuMDU0IDI0My41MjUgNTk0LjE5NyAyODcuNTg0IDU5NC4xOTdINjAwTDQ0My43OTIgNDM4Ljc3OUgyNjkuMTU5TDU0OC43MzIgMTYwLjYyMUw0NzAuMjI3IDgyLjUxMzZDNDA2LjE0MiAxOC43NTIyIDM0Mi4wNTYgNS45OTk5NiAzMjIuMDMgNloiIGZpbGw9IndoaXRlIi8+Cjwvc3ZnPgo=)
![GitHub License](https://img.shields.io/github/license/zmann-org/zmann?style=for-the-badge&labelColor=000)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/zmann-org/zmann/total?style=for-the-badge&labelColor=000)
![Static Badge](https://img.shields.io/badge/VST3-C90827?style=for-the-badge&logo=steinberg&labelColor=000)

## Plugins
As defined in [bundler.toml](./bundler.toml), the following plugins are included in this repository, for more information on each plugin, please refer to their respective product pages on **zmann.org**.

|Product|CI/CD|TYPE||
|---|---|---|---|
|[Toybox C1200](https://zmann.org/product/toybox-c1200)|![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/zmann-org/zmann/toybox_c1200.yml?style=for-the-badge&labelColor=000)|![Series 1](https://img.shields.io/badge/Series%201-0072f5?style=for-the-badge)|[![Download Button](./.github/icons/download-badge.svg)](https://github.com/zmann-org/zmann/releases)|
|Orchestron|![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/zmann-org/zmann/orchestron.yml?style=for-the-badge&labelColor=000)|![Series 1](https://img.shields.io/badge/Series%201-0072f5?style=for-the-badge)|[![Download Button](./.github/icons/download-badge.svg)](https://github.com/zmann-org/zmann/releases)|
|[Bilhorn Reed](https://zmann.org/product/bilhorn-reed)||![Series 1](https://img.shields.io/badge/Series%201-0072f5?style=for-the-badge)|[![Download Button](./.github/icons/download-badge.svg)](https://github.com/zmann-org/zmann/releases)|
|Hohner Melodica||![Series 1](https://img.shields.io/badge/Series%201-0072f5?style=for-the-badge)|[![Download Button](./.github/icons/download-badge.svg)](https://github.com/zmann-org/zmann/releases)|
|Mellotron||![Series 1](https://img.shields.io/badge/Series%201-0072f5?style=for-the-badge)||
|   |   |   |
|NoiseG8|   |![Tool](https://img.shields.io/badge/DAW%20Tool-orange?style=for-the-badge)||


## Installation

After downloading the *.vst3* file, move it to your VST3 directory. The default VST3 directory is located at `C:\Program Files\Common Files\VST3` on Windows. If you are unsure where your VST3 directory is located, you can check the VST3 directory in your DAW's settings.

> [!TIP]
> Create a dedicated folder within your VST3 directory, e.g., `ZMANN`, for better organization.

## Table of Contents
- [Plugins](#plugins)
- [Installation](#installation)
- [Demo](#demo)
- [Documentation](#documentation)
- [Building](#building)
  - [Prerequisites](#prerequisites)
  - [Building the plugins](#building-the-plugins)
  - [Cross-compiling](#cross-compiling)
- [Acknowledgments](#acknowledgments)

## Building
> [!WARNING]  
> As of writing, only Windows is supported for building. Linux cross-compilation might work, but is not stable at this time. Currently we don't have any motivation in supporting Mac OS, although the plugins should be compatible without many modifications. [Read more about cross-compilation](#cross-compiling).
### Prerequisites
- [Node.js 18.17](https://nodejs.org/en/) or later
- [Rust](https://www.rust-lang.org/tools/install)

#### Windows
Before building, make sure that rustup is installed with **msvc** if building on Windows:
```bash
$ rustup default stable-x86_64-pc-windows-msvc
```

### Adding the artifacts
As of writing, we aren't including the samples (for example *.binv5* or *.microbin*) in the repository due to the git file limit. For the builds not to fail, supply a `/samples/plugin_name` folder, where `plugin_name` is the [name of the plugin](#plugins) you are building. You can create the folder by running:
```bash
$ mkdir /samples/plugin_name
```
Then, add the samples to the folder. The folder structure should look like this:

<!--use css formatting to demonstrate file extension-->
```css
/samples
  /Toybox_C1200
    /Cello.binv5
```
Release builds will be built with the samples included in the plugin bundle.

### Building the plugins
Plugins are required to be built separately from each other. Run the *xtask build* command with the [plugin name](./bundler.toml) and `--release` at the end to target a release build. For example:
```bash
$ cargo xtask bundle toybox_c1200 --release
```
Cargo will now build the *toybox_c1200_ui* before building the plugin itself. After successfully building, each plugin will make their own *.vst3* bundle inside of the `/target/bundled/` folder. 

Or to build all projects inside of the repository, run to build all the ui and plugins in the repository:
```bash
$ cargo xtask bundle -p toybox_c1200 -p orchestron 
```


Optionally by running mklink on Windows, a simlink will be created from the locally generated artifacts folder to the standard vst3 folder to streamline development. This will allow you to test the plugins in your DAW without having to move the files manually. Run the following command in the root of the repository:
```bash
mklink /j "%COMMONPROGRAMFILES%\VST3\zmann-dev" "%~dp0target\bundled\"
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
$ cargo xtask bundle plugin_name
```

## Acknowledgments
- [**nih-plug-webview**](./crates/nih_plug_webview/) (Modified)
  - Original Library: [nih-plug-webview](https://github.com/maxjvh/nih-plug-webview)
  - License: ISC

- [**fx**](./crates/fx/) (Modified)
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
