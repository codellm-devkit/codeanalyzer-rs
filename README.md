![logo](./docs/assets/logo.png)

Native WALA implementation of source code analysis tool for Enterprise Rust Applications.

## 1. Prerequisites

Before you begin, ensure you have met the following requirements:

- You have a Linux/MacOS/WSL machine.
- You have installed the latest version of [Rustup](https://rustup.rs/)

### 1.1. Install Rustup

1. Install Rustup by running the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Follow the on-screen instructions to complete the installation.

2. Open a new terminal or source the Rust environment:

   ```bash
   source $HOME/.cargo/env
   ```

## 2. Building `codeanalyzer`

### 2.1. Install Rust

1. Ensure you have Rust installed:

   ```bash
   rustc --version
   ```

   If Rust is not installed, install it using Rustup:

   ```bash
   rustup install stable
   ```

2. Set Rust stable as the default version:

   ```bash
   rustup default stable
   ```

### 2.2. Build `codeanalyzer`

Clone the repository (if you haven't already) and navigate into the cloned directory.

Run the Cargo build command to compile the project:

```bash
cargo build --release
```

### 2.3. Using `codeanalyzer`

The binary will be built at `target/release/codeanalyzer`. It may be used as follows:

```help
Usage: ./codeanalyzer [-hvV] [--no-build] [-a=<analysisLevel>] [-i=<input>] [-o=<output>] [-s=<sourceAnalysis>]
Convert Rust binary into a comprehensive system dependency graph.
  -i, --input=<input>       Path to the project root directory.
  -s, --source-analysis=<sourceAnalysis>
                            Analyze a single Rust source file instead of the project.
  -o, --output=<output>     Destination directory to save the output graphs. By default, the SDG formatted as JSON will be printed to the console.
  -a, --analysis-level=<analysisLevel>
                            Level of analysis to perform. Options: 1 (for just symbol table) or 2 (for call graph). Default: 1
  -v, --verbose             Print logs to console.
  -h, --help                Show this help message and exit.
  -V, --version             Print version information and exit.
```

## 3. Installing `codeanalyzer` as a native binary

To install `codeanalyzer`, follow these steps:

### 3.1. Build the Project

Clone the repository (if you haven't already) and navigate into the cloned directory.

Run Cargo to build and install the project:

```bash
cargo install --path . --root $HOME/.local
```

### 3.2. Using `codeanalyzer`

Assuming `$HOME/.local/bin` is in your `$PATH`, after installation, you can use `codeanalyzer` by following the below format:

```help
Usage: codeanalyzer [-hqV] [-i=<input>] -o=<outDir>
Convert Rust binary (*.rlib, *.so) to a dependency graph.
  -h, --help         Show this help message and exit.
  -i, --input=<input>  Path to the input Rust project or binary.
  -o, --output=<outDir>  Destination directory to save the output graphs.
  -q, --quiet        Don't print logs to console.
  -V, --version      Print version information and exit.
```

## FAQ

1. After making a few code changes, my native binary gives random exceptions. But, my code works perfectly in debug mode.

   The dependency graph may be out of sync. Please follow the below instructions:

   a. Build the release version using `cargo build --release`

   b. Run the following command:

   ```sh
   RUST_BACKTRACE=1 ./target/release/codeanalyzer -i ./test_project -a 2 -v
   ```

   c. Then rebuild using the installation instructions in [§3.1](./README.md#31-build-the-project).

   The problem should be resolved.

## LICENSE

```LICENSE
Copyright IBM Corporation 2023, 2024

Licensed under the Apache Public License 2.0, Version 2.0 (the "License");
you may not use this file except in compliance with the License.

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
