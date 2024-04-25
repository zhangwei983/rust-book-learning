Tutorial about how to install Rust.

Here we use `rustup` (the Rust toolchain installer) to install Rust. Personally I've installed Rust on Windows and WSL2 with Ubuntu.

## On Linux and macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup` tool, which installs the latest stable version of Rust. 

### C compiler

On Mac, you can get a C compiler by running:

```bash
$ xcode-select --install
```

On Linux, you generally install `GCC` or `Clang`, according to their distribution’s documentation.

I'm on `WSL2 with Ubuntu`, so I installed the `build-essential` package.

## On Windows

On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. 

### MSVC

And you need to install `MSVC` build tools, please install [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/) with below components installed:

- “Desktop Development with C++”
- The Windows 10 or 11 SDK
- The English language pack component, along with any other language pack of your choosing.

Actually, I've already have `Microsoft Visual Studio Community 2022` installed before, so this is not a problem to me.

## Verification

After installation, you can run

```bash
$ rustc --version
```

to check if Rust has been installed successfully.

```bash
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```

This is what I got after running `rustc --version` on my Windows. 

If you didn't see it, please check if Rust is in your `environment path`.

## Updating and Uninstalling

And you can run 

```bash
$ rustup update
```

to update Rust to the latest stable version.

Also you can run 

```
$ rustup self uninstall
```

to uninstall Rust if you cannot find the spark of love between you and Rust...

## Local Documentation

Run

```bash
rustup doc
```

to open the local documentation installed along with Rust.
