# FUS operator (`wb55_operator.hex`)

The Wireless Stack tab of EasyFlash does not talk to the STM32WB55 FUS directly. It first flashes a small CODAL program, the **operator**, on the STM32WB55. The operator reads one command per line on the serial port (115200 baud) and answers with one JSON line. It is not the operator from STMicroelectronics.

EasyFlash uses two files from [`wireless_stack/`](../wireless_stack):

| File | Used for |
|------|----------|
| `wb55_operator.hex` | Flashed alone (step "Flash operator"), then EasyFlash sends `VERSION`, `STATUS`, `DELETE` and `UPGRADE`. |
| `wb55_operator_no_end.hex` | The same file without its last line (the Intel HEX end-of-file record `:00000001FF`). EasyFlash puts it in front of the FUS or wireless stack file to build `merge.hex`, so that OpenOCD flashes both in one go. |

Both files are in this repository and in the release archives. EasyFlash does not download anything at run time.

This folder holds the sources of the operator, so that it can be read and rebuilt without any other STeaMi repository.

## Commands

| Command | Action | Answer |
|---------|--------|--------|
| `VERSION` | Reads the FUS and wireless stack versions | `{"command": "Version", "status": 0, "fus_version": …, "copro_fw_version": "x.y.z", "ws_version": …}` |
| `STATUS` | Calls `SHCI_C2_FUS_GetState` and reads the head of the device information table | `{"command": "Status", "status": …, "last_fus_status": …, "last_ws_status": …, "current_ws": …}` |
| `DELETE` | Calls `SHCI_C2_FUS_FwDelete` (deletes the wireless stack) | `{"command ": "Delete", "status": …}` |
| `UPGRADE` | Calls `SHCI_C2_FUS_FwUpgrade`, then polls `SHCI_C2_FUS_GetState` every 100 ms until it returns 0 | `{"command": "Upgrade", …}` then `{"command": "Upgrade(status)", …}` lines |
| `START` | Calls `SHCI_C2_FUS_StartWs` | `{"command": "Start", "status": …}` |
| `RESET` | Resets the STM32WB55 | none |
| `HELP` | Prints the list of commands | text |

Commands are case insensitive. The `"command "` key of the `DELETE` answer has a trailing space in the source; EasyFlash does not read that key.

## Provenance

The files in this folder are byte-for-byte copies of the files of the same path in the `steamicc/codal-steami-samples` repository, commit `47c3b989165c197ddeaf60c334f721ac2b421a8b` (2024-10-30, the last commit of its `main` branch). That repository is going to be archived; this folder replaces it as the reference for the operator.

| File here | Git blob |
|-----------|----------|
| `samples/Peripherals/FUS_WS_Operator/FUS_WS_Operator.cpp` | `1844b7b12d5aefe6555f6a675f0137ef7548bc28` |
| `samples/Peripherals/FUS_WS_Operator/FUS_WS_Operator.h` | `44186235d863b690ac934fd9d0d196909a789349` |
| `samples/main.cpp` | `edb759159cd54701bd7b4c1d60244d8e72349cf6` |
| `CMakeLists.txt` | `333fa81fb5533abbf2884af7c820db152c830dfd` |
| `target.json` | `5bd4cbb0204349583614c39151023e4bc9ce17d9` |
| `target-locked.json` | `91d322f4bc013e97045bc8edf07c5cad0939c34b` |
| `LICENSE` | `a38b82ead6a694c21045c84be60ed9b346da4e1b` |

The two operator files have not changed since the first commit of that repository (`212ab1679e4d8393a501397c4dc591f579ebc739`, 2024-10-29). `samples/main.cpp`, `CMakeLists.txt`, `target.json` and `target-locked.json` make this folder a CODAL target, as it was in the original repository. Only the operator sample is kept: `samples/main.cpp` still lists the other samples, but with `FUS_WS_OPERATOR_SAMPLE` defined it only includes `FUS_WS_Operator.h`.

### The hex file and these sources

The hex files were added to EasyFlash in commit `535758d` (2025-04-23). Their SHA-256 sums are:

```
0801ec7ec78dff3791061a748d64bd893ffc36a9c1c68b0df561a11037c56057  wireless_stack/wb55_operator.hex
81b2517f3e6e3752e740315de1c368326ecdada714dc03b3a087ab03f8a6d561  wireless_stack/wb55_operator_no_end.hex
```

`wb55_operator.hex` was built from a slightly different version of these sources. It contains every message of `FUS_WS_Operator.cpp`, and also the message `Unknown command '%s'`, right after the `HELP` text. That message is not in `FUS_WS_Operator.cpp`, nor in the default branches of the CODAL libraries listed in `target.json`. The source of that version has not been published. A build of this folder gives the same commands and the same answers, but it does not print a message for an unknown command, and the hex file is not identical.

## License

- The files of this folder: MIT license, copyright (c) 2024 STeaMi Educational Foundation (see [`LICENSE`](LICENSE)). The repository they come from has the same license.
- These sources contain no file from STMicroelectronics. They include headers of the CODAL libraries, which are not copied here.
- The hex file also contains code compiled from those libraries:
  - [letssteam/codal-core](https://github.com/letssteam/codal-core): MIT license.
  - [letssteam/codal-stm32](https://github.com/letssteam/codal-stm32): BSD 3-Clause license. It includes STMicroelectronics code under its own licenses (`system/` and `inc/drivers/Bluetooth/STM32_WPAN_WB55/` in that repository). The FUS commands (`shci.h`, `shci.c`) come from `STM32_WPAN_WB55`, under the STMicroelectronics SLA0044 license (`inc/drivers/Bluetooth/STM32_WPAN_WB55/LICENSE.md`). SLA0044 requires this code to run only on or with an STMicroelectronics microcontroller (clause 4), which is the case for the STM32WB55, and does not require its notice in a binary embedded in such a microcontroller or used as a software update for it (clause 2). It also forbids putting that code under an open source license (clause 5): the MIT license above covers the files of this folder, not the hex file.
  - [letssteam/codal-stm32-STEAM32_WB55RG](https://github.com/letssteam/codal-stm32-STEAM32_WB55RG): MIT license, board support for the STeaMi.
  - newlib and libstdc++ from the Arm GNU toolchain.

## Rebuild

The build uses the CODAL build tool ([letssteam/codal](https://github.com/letssteam/codal)) and the Docker image `letssteam/makecode-toolchain:latest` (Arm GNU toolchain, CMake, Python), the same as the original continuous integration. These steps have not been run again since the copy.

```sh
git clone https://github.com/letssteam/codal.git
cd codal
mkdir -p libraries
cp -r /path/to/DapLink-EasyFlash/fus-operator libraries/codal-steami-samples
cat > codal.json <<'EOF'
{
  "target": {
    "name": "codal-steami-samples",
    "url": "https://github.com/steamicc/DapLink-EasyFlash",
    "branch": "main",
    "type": "git"
  },
  "config": {
    "FUS_WS_OPERATOR_SAMPLE": 1
  }
}
EOF
docker run --rm -v "$PWD":/codal -w /codal letssteam/makecode-toolchain:latest ./build.py
```

- The target folder must be named `codal-steami-samples`, because `CMakeLists.txt` declares a library of that name. CODAL does not clone the target when `libraries/codal-steami-samples` already exists, so `url` is not used.
- Without `"dev": true`, CODAL reads `target-locked.json` and checks out the CODAL libraries at the commits it lists (codal-core `02c59a8`, codal-stm32 `bfdb5c8`, codal-stm32-STEAM32_WB55RG `d281ea2`). With `"dev": true` (as in the original continuous integration), it reads `target.json` and takes the `main` branches.
- The result is `STM32.hex` at the root of `codal/`. Copy it to `wireless_stack/wb55_operator.hex`, then build the variant without the end-of-file record:

```sh
head -n -1 wireless_stack/wb55_operator.hex > wireless_stack/wb55_operator_no_end.hex
```

The last line of `wb55_operator.hex` must be `:00000001FF`. Check this before removing it.
