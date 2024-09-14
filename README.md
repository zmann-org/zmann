![zmann logo](.github/icons/logo-dark.png#gh-dark-mode-only)
![zmann logo](.github/icons/logo-light.png#gh-light-mode-only)

![GitHub License](https://img.shields.io/github/license/zmann-org/zmann?style=for-the-badge&labelColor=000)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/zmann-org/zmann/total?style=for-the-badge&labelColor=000)
![Static Badge](https://img.shields.io/badge/VST3-C90827?style=for-the-badge&logo=steinberg&labelColor=000)
> [!IMPORTANT]  
> We're working on a restructure of the repository, and planning on switching to a different GUI for the plugins. Read more at [#477](https://github.com/zmann-org/zmann/issues/477)

<h1 align="center">

![gh-banner](https://raw.githubusercontent.com/zmann-org/zmann/main/.github/marketing/header-transparent-crop.png)

</h1>



## Plugins
As defined in [bundler.toml](./bundler.toml), the following plugins are included in this repository, for more information on each plugin, please refer to their respective product pages on **zmann.org**.

|Product|Status|||
|---|---|---|---|
|Toybox C1200||![Series 1](https://img.shields.io/badge/Series%202-0072f5?style=for-the-badge)||
|Orchestron|WIP|![Series 1](https://img.shields.io/badge/Series%202-0072f5?style=for-the-badge)||
|Bells||![Series 1](https://img.shields.io/badge/Series%202-0072f5?style=for-the-badge)||
|Hohner Melodica||![Series 1](https://img.shields.io/badge/Series%202-0072f5?style=for-the-badge)||
|Mellotron||![Series 1](https://img.shields.io/badge/Series%202-0072f5?style=for-the-badge)||
|   |   |   |
|NoiseG8||![Tool](https://img.shields.io/badge/DAW%20Tool-orange?style=for-the-badge)||


## Installation

For VST3 plugins, unzip and move the folder ending in `.vst3` into the VST3 directory. Normally this is set to `C:\Program Files\Common Files\VST3` on Windows. If you are unsure where your VST3 directory is located, you can check the VST3 directory in your DAW's settings.

> [!TIP]
> You can create a dedicated folder within the VST3 directory, e.g., `ZMANN`, for better organization.

For CLAP plugins download and move the `.clap` file into `C:\Program Files\Common Files\CLAP`. (or the custom folder that you have set up in your DAW)

## Table of Contents
- [Plugins](#plugins)
- [Installation](#installation)
- [Demo](#demo)
- [Documentation](#documentation)
- [Building](#building)
  - [Prerequisites](#prerequisites)
  - [Building the plugins](#building-the-plugins)
  - [Cross-compiling](#cross-compiling)
- [License](#license)

## Demo


## Building
> [!WARNING]  
> Currently we don't have any plans to support Mac OS, although the plugins should work after compiling to a Mac OS target. [Read more about cross-compilation.](#cross-compiling).
### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)

#### Windows
Before building, make sure that rustup is installed with **msvc** if building on Windows:
```bash
$ rustup default stable-x86_64-pc-windows-msvc
```

### Building the plugins
Plugins are required to be built separately from each other. Run `cargo xtask build` with the [plugin name](./bundler.toml) and `--release` at the end to target a release build. For example:
```bash
$ cargo xtask bundle orchestron --release
```
After successfully building, each plugin will make their own VST3 and CLAP bundle inside of the `/target/bundled/` folder. 

Alternatively, build all plugins inside of the repository by running:
```bash
$ cargo ci
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
$ cargo ci
```

## License
The code in this repository is licensed under the GNU General Public License v3.0 or later. You can find a copy of the license in the [LICENSE](./LICENSE) file.

The samples included in this repository are owned by their respective owners. Please refer to the individual sample files for their specific licensing information.

VST® is a trademark of Steinberg Media Technologies GmbH, registered in Europe and other countries. Other company names, product names and logos are the trademarks or registered trademarks of their respective owners.
