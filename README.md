# Blue Archive - Flatbuffers
A tool for dumping and generating Blue Archive flatbuffers


## Install

### Release
You can download the latest pre-build binaries at [Releases](https://github.com/Deathemonic/BA-FB/releases)

[Windows](https://github.com/Deathemonic/BA-FB/releases/latest/download/bafb-windows-x86_64.zip) | [Linux](https://github.com/Deathemonic/BA-FB/releases/latest/download/bafb-linux-x86_64.zip) | [MacOS](https://github.com/Deathemonic/BA-FB/releases/latest/download/bafb-macos-aarch64.zip)

### Cargo
```shell
cargo install --git "https://github.com/Deathemonic/BA-FB" --locked
```

## Usage

```shell
# Force update all tools and APKs
bafb --update

# Cleans everything
baad --clean

# Dump Blue Archive flatbuffers from Japan server
bafb dump japan -o ./output

# Dump Blue Archive flatbuffers from Global server  
bafb dump global -o ./output

# Generate Rust code from flatbuffer schema
bafb generate -f BlueArchive.fbs -l rust -o ./generated

# Use custom configuration file
bafb dump japan -o ./output --config ./my_configs.toml
```

<details>
  <summary>Command Line</summary>

| Command/Option | Short | Description                                               |
|----------------|-------|-----------------------------------------------------------|
| `dump`         |       | Dump Blue Archive flatbuffers                             |
| `generate`     |       | Generate code from flatbuffer schema                      |
| `help`         |       | Print this message or the help of the given subcommand(s) |
| `--config`     |       | Path to configuration file (defaults to `./config.toml`)  |
| `--update`     | `-u`  | Force update all tools and APK files                      |
| `--clean`      | `-c`  | Cleans the cache                                          |
| `--help`       | `-h`  | Print help                                                |
| `--version`    | `-V`  | Print version                                             |

---

### `bafb dump --help`

| Command  | Description                                               |
|----------|-----------------------------------------------------------|
| `japan`  | Dump from Japan server                                    |
| `global` | Dump from Global server                                   |
| `help`   | Print this message or the help of the given subcommand(s) |

---

### `bafb dump {japan|global} --help`

| Option              | Short | Description                           | Default |
|---------------------|-------|---------------------------------------|---------|
| `--output <OUTPUT>` | `-o`  | Output directory for dumped files     |         |
| `--help`            | `-h`  | Print help                            |         |

---

### `bafb generate --help`

| Option                | Short | Description                                                                                                                                                           | Default |
|-----------------------|-------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------|
| `--fbs <FBS>`         | `-f`  | FlatBuffers schema file (.fbs)                                                                                                                                        |         |
| `--language <LANG>`   | `-l`  | Target language                                                                                                                                                       |         |
| `--output <OUTPUT>`   | `-o`  | Output directory for generated code                                                                                                                                   |         |
| `--help`              | `-h`  | Print help                                                                                                                                                            |         |

**Supported Languages:** `cpp`, `java`, `kotlin`, `kotlin-kmp`, `csharp`, `go`, `python`, `javascript`, `typescript`, `php`, `dart`, `lua`, `lobster`, `rust`, `swift`, `nim`

</details>

## Building

1. Install [rustup](https://rustup.rs)
2. Clone this repository
```sh
git clone https://github.com/Deathemonic/BA-FB
cd BA-FB
```
3. Build using `cargo`
```sh
cargo build
```

### Other Projects

- [BA-AD](https://github.com/Deathemonic/BA-AD): A tool and library that downloads the latest **Blue Archive** assets
- [BA-AX](https://github.com/Deathemonic/BA-AX): A tool and library that extracts **Blue Archive** assets
- [BA-MU](https://github.com/Deathemonic/BA-MU): A tool that re-dump AssetBundle for **Blue Archive**.
- [BA-CY](https://github.com/Deathemonic/BA-CY): Library for handling **Blue Archive** catalogs, tables, serialization/deserialization, encryption, and hashing.


### Contributing
Don't like my [shitty code](https://www.reddit.com/r/programminghorror) and what to change it? Feel free to contribute by submitting a pull request or issue. Always appreciate the help.


### Dependencies
BA-FB relies on this tools in order for it to work.

- [ArkanDash/FbsDumper](https://github.com/ArkanDash/FbsDumper) - Generates Flatbuffers schema.
- [LukeFZ/Il2CppInspectorRedux](https://github.com/LukeFZ/Il2CppInspectorRedux) - Generates the il2cpp .NET dlls.
- [google/flatbuffers](https://github.com/google/flatbuffers) - Compiles the Flatbuffers schema to various languages.

### Acknowledgement
- [ArkanDash/BA-Source-Dump](https://github.com/ArkanDash/BA-Source-Dump)
- [Hiro420/FbsDumper](https://github.com/Hiro420/FbsDumper)

### Copyright
Blue Archive is a registered trademark of NAT GAMES Co., Ltd., NEXON Korea Corp., and Yostar, Inc.
This project is not affiliated with, endorsed by, or connected to NAT GAMES Co., Ltd., NEXON Korea Corp., NEXON GAMES Co., Ltd., IODivision, Yostar, Inc., or any of their subsidiaries or affiliates.
All game assets, content, and materials are copyrighted by their respective owners and are used for informational and educational purposes only.
