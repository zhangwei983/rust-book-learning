An example to use protobuf to read and write protocol buffers data.

## Install

### Install protobuf compiler

Follow this [instruction](https://grpc.io/docs/protoc-installation/) to install protobuf compiler on different platforms.

For Windows, 
- Please go to this release [page](https://grpc.io/docs/protoc-installation/) to download the win32 or win64 zip file.
- Unzip the zip file and locate the `protoc.exe` file.
- Set the environment variable with 
  - name: `PROTOC` 
  - value: `C:\tools\protoc-29.1-win64\bin\protoc.exe`
    This is an example of the value, make sure you set the path with the binary.

### Install crates

We use `prost_build` to compile `.proto` files in to Rust.

- Install the `prost` crate  
  `cargo add prost`

- Install the `prost-build` create  
  `cargo add --build prost-build`
