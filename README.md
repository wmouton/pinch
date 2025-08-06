# Pinch

Compress and extract files with .tar.gz extensions. I created this CLI tool for the single purpose of compressing and extracting my dotfiles.

# Installation

The only way to install `pinch` currently is by building it directly from the source code. This guide will walk you through the process.

## Prerequisites

Before you begin, you'll need to have a few tools installed on your system.

  * **Git**: Required to clone the project repository from GitHub. You can download it from [git-scm.com](https://git-scm.com/download/).
  * **Rust Toolchain**: Required to compile the project. You can install it easily by visiting [rustup.rs](https://rustup.rs/). This will install both `rustc` (the compiler) and `cargo` (the package manager).
  * **tar**: `pinch` is a wrapper around the `tar` command.
      * **Linux & macOS**: `tar` is almost always pre-installed.
      * **Windows**: The easiest way to get `tar` is by installing [Git for Windows](https://git-scm.com/download/win), which includes `tar` and other common command-line tools.

-----

## Installation Steps

Once the prerequisites are met, you can proceed with the installation.

### 1\. Clone the Repository

Open your terminal and use `git` to clone the `pinch` repository to your local machine.

```bash
git clone https://github.com/wmouton/pinch.git
```

### 2\. Navigate to the Project Directory

Change into the newly created `pinch` directory.

```bash
cd pinch
```

### 3\. Build and Install the Binary

Use `cargo` to compile the project and install the executable. The `cargo install` command will build the project in release mode (`--release`) and copy the final binary to your `~/.cargo/bin` directory, which should be in your system's `PATH`.

```bash
cargo install --path .
```

After the command finishes, the `pinch` executable is installed and ready to be used from anywhere in your terminal.

-----

## Post-Installation Verification

To confirm that `pinch` was installed correctly, open a **new** terminal session and run the following command to check its version.

```bash
pinch --version
```

You should see an output like this:
`pinch 0.1.0`

You can also view the help menu for a full list of commands and options.

```bash
pinch --help
```

You're all set\! You can now use `pinch` to compress and extract `.tar.gz` archives.
