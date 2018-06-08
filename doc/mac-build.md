# Setup Indy SDK build environment for MacOS

`libindy` provides the c-callable interface to which all wrappers and the `indy-cli` connect. Since it is the most basic
building block for creating applications that use Hyperledger Indy, you will need to install it first.

## Building `libindy`
### 1. Install Rust

Installation via `rustup` is recommended. Follow [these instructions](https://www.rust-lang.org/install.html).

Make sure that `cargo` is in your `PATH`.

### 2. Install required native libraries and utilities

It is recommended that you install all dependencies through `homebrew`.

You can find instructions for how to install `homebrew` [here](https://brew.sh/).

```sh
	brew install pkg-config
	brew install libsodium
	brew install automake
	brew install autoconf
	brew install cmake
	brew install openssl
	brew install zeromq
```

If you run into issues with 

### 3. Setup environment variables:

export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_INCREMENTAL=1
export RUST_LOG=indy=trace
export RUST_TEST_THREADS=1

### 4. Setup `OPENSSL_DIR` variable

Without this environment variable being set, `cargo` can not find OpenSSL. Anything that depends on this library will
fail.

```sh
export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2n   # path changes with version number
```

**Note that the directory will be different based on the version of `openssl` installed.**

### 5. Checkout and build the library

```sh
git clone https://github.com/hyperledger/indy-sdk.git
cd ./indy-sdk/libindy
cargo build
```

### 6. Setup Docker

Docker is used to create 4 Ubuntu instances running the Indy Ledger for testing purposes. In practice, you would
connect to an actual Sovrin Network. The simple docker test pool allows us to test our installation of `libindy`.

Follow the instructions [here](https://store.docker.com/editions/community/docker-ce-desktop-mac) to install
Docker for MacOS.

Then build and run the docker files with the following commands (from the `indy-sdk` directory):

```sh
docker build -f ci/indy-pool.dockerfile -t indy_pool .
docker run -itd -p 9701-9708:9701-9708 indy_pool
```

**Note:** In order to run local nodes on MacOS, it may be necessary to set up port mapping between the Docker container
and local host. Follow the instructions in the
[Indy SDK README](https://github.com/hyperledger/indy-sdk#how-to-start-local-nodes-pool-with-docker)

### 7. Run tests
```sh
RUST_TEST_THREADS=1 cargo test
```

# Building and Running `indy-cli`

`indy-cli` is not required to create applications on top of `libindy` but may be a helpful tool in development as it
provides a direct interface to wallets, pools, etc.

First enter the proper directory from the `indy-sdk` directory: `cd cli`

## Build `indy-cli`

TODO: Say more about RUSTFLAGS

```sh
RUSTFLAGS="-L ../libindy/target/debug" cargo build
```

# Running the Python Getting Started Guide

```sh
cd samples/python
```

## Install Python 3

```sh
brew install python3
```

## Downgrade pip 9.0.3

Might not be needed

```sh
pip3 install --upgrade pip==9.0.3
```

## Create Virtual Environment

```sh
python3 -m venv env
source env/bin/activate
```

## Install Dependencies

```sh
pip install .
```

## Export `LD_LIBRARY_PATH`
```sh
export LD_LIBRARY_PATH=/path/to/indy-sdk/libindy/target/debug
```

## Run the Guide

```sh
python -m src.main
```
