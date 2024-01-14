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
|[Toybox C1200](./crates/toybox_c1200/)|![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/zmann-org/zmann/monorepo.yml?style=plastic)|![pluginval](https://img.shields.io/badge/pluginval-passed-green.svg?style=plastic&logo=data:image/svg%2bxml;base64,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/YmGlS5O6MoqkBcEVhHq27evee655wgtoYsRuiihK6pAXhFYRaxfv37m2WeftXvxlAwrXUkF8oPAKgb9+/ePbWglw0pXUIH8IrCKyYABA2IXWhpWuvhAWKGgCKxipKH1zDPP2L3MpmGliw66YgoUFIFVzAYOHGieeuopu5eZdJFBw0pXSoHCILAccO+992ZsaGlY6cooYYVUILAcoaH1z3/+0+5lhmRY6cookAoElkPuv//+jAktDStdVCCskEoElmM0tJ544gm75ycNq6effjq8fQNIJQLLQQ8++KDXoaVhpYsJQKoRWI7S0Hr88cftnj908YCwQroQWA576KGHzGOPPWb33KdhpYsHQLoQWI57+OGHzaOPPmr33KWLBYQV0o3A8sAjjzwSNldpWOliAZBuBJYndJSloy3XPPnkk4QVigyB5RGdz9J5LVfoSuYDDzxg94D0I7A8oyuHuoJY3Fx5HogXAstDxT2ycW2kh/ggsDxVXHNHrs6lIR4ILI/p6tx9991n99LP9dVKZL6sQNjH8JSGlv45TDrpqMqnm1iRmRhhZYB032Hu2x33yFwEVobQ0NLJeH0r4lTRzw3Ut3D28W8akZl4SZhhpk2bFr5v+o4dO2ylYMqXLx9+fHy7du1sBSh+jLAyTMeOHc26detM69atbSX/2rRpE34NwgquYYSVoXJzc83ixYvNyJEjzfr1683+/fvtv0QrXbq0adGiRfhJzK1atQrfhA9wDYEVA1u2bDFr1qwxGzZsMF9++aXZu3dvWC9btqypVauWadKkibn66qtN/fr1wzrgKgIrZnTktW/fvnAEVaZMmZRO0gPpRmAB8Aa/XgF4g8AC4A0CC4A3CCwA3iCwAHiDwALgDQILgDcILADeILAAeIPAAuANAguANwgsAN4gsAB4g8AC4A0CC4A3CCwA3iCwAHiDwALgDQILgDcILADeILAAeIPAAuANAguANwgsAN4gsAB4g8AC4A0CC4A3CCwA3iCwAHiDwALgDQILgCeM+X+dAkzrJCvg8gAAAABJRU5ErkJggg==)|[Download](https://github.com/zmann-org/zmann/releases)|
|   |   |   |   |

for more information on each plugin, please refer to their respective product pages on the zmann website.

## Installation
> [!TIP]
> You can make a ZMANN folder inside of your VST3 folder to keep your ZMANN plugins all in one place.

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
### Prerequisites
> **Warning**
> As of writing, only Windows is supported for building. Linux cross-compilation might work, but is not stable at this time. [Read more about cross-compilation](#cross-compiling).
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

### Cross Compiling
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