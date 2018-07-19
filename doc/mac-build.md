# Setup Indy SDK build environment for MacOS

`libindy` provides the c-callable interface to which all wrappers and the `indy-cli` connect. Since it is the most basic
building block for creating applications that use Hyperledger Indy, you will need to install it first..

## Building `libindy`
### 1. Install Rust

Installation via `rustup` is recommended. Follow [these instructions](https://www.rust-lang.org/install.html).

Make sure that `cargo` is in your `PATH` (`source $HOME/.cargo/env`).

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

If you run into issues with `libzmq`, it may help to also `brew install zmq`.

### 3. Setup `OPENSSL_DIR` variable

Without this environment variable being set, `cargo` cannot find OpenSSL. Anything that depends on this library will
fail.

```sh
export OPENSSL_DIR=/usr/local/Cellar/openssl/1.0.2n   # path changes with version number
```

**Note that the directory will be different based on the version of `openssl` installed.**

### 4. Checkout and build the library

```sh
git clone https://github.com/hyperledger/indy-sdk.git
cd ./indy-sdk/libindy
cargo build
```

### 5. Setup Docker

Docker is used to create 4 Ubuntu instances running the Indy Ledger for testing purposes. In practice, you would
connect to an actual Sovrin Network. The simple docker test pool allows us to test our installation of `libindy`.

Follow the instructions [here](https://store.docker.com/editions/community/docker-ce-desktop-mac) to install
Docker for MacOS (and make sure Docker is running).

Then build and run the docker files with the following commands (from the `indy-sdk` directory):

```sh
docker build -f ci/indy-pool.dockerfile -t indy_pool .
docker run -itd -p 9701-9708:9701-9708 indy_pool
```

**Note:** If you run into issues, try following the instrutions in this link. In order to run local nodes on MacOS,
it may be necessary to set up port mapping between the Docker container and local host. Follow the instructions in the
[Indy SDK README](https://github.com/hyperledger/indy-sdk#how-to-start-local-nodes-pool-with-docker)

### 6. Run tests
```sh
RUST_TEST_THREADS=1 cargo test
```
The tests may take awhile to complete.

# Building and Running `indy-cli`

`indy-cli` is not required to create applications on top of `libindy` but may be a helpful tool in development as it
provides a direct interface to wallets, pools, etc.

First enter the proper directory from the `indy-sdk` directory: `cd cli`

## Build `indy-cli`

```sh
RUSTFLAGS="-L ../libindy/target/debug" cargo build
```
`indy-cli` is dependent on `libindy` and requires additional flags during build to help it find the correct
library. The `RUSTFLAGS` variable specifies an additional directory where `cargo` should look for library
dependencies.

The genesis transaction block is used as the first block on the indy ledger. It is used by `indy-cli` but must be
modified to run on MacOS. Open `docker_pool_transactions_genesis`, and replace each of 8 occurences of 
`10.0.0.2` with `127.0.0.1`

Run the the cli:

```sh
target/debug/indy-cli
```

# Running the Python Getting Started Guide

```sh
cd samples/python
```

## Install Python 3

```sh
brew install python3
```

## Create Virtual Environment

A Python virtual environment is used keep a local copy of all the package dependencies that are needed for a particular project without modifying the global packages. This keeps things cleaner and helps prevent errors.

```sh
python3 -m venv env
source env/bin/activate
```

From this point forward, you can get the right version of Python with `python` and `pip` (no 3 required) because they now refer to the virtual environment's copy of Python.

## Install Dependencies

`pip` will look in `setup.py` for dependencies and install them into the virtual environment from the previous step.

```sh
pip install .
```

If you have any issues installing dependencies using pip, try downgrading/upgrading to version `9.0.3`:
```sh
pip install --upgrade pip==9.0.3
```

## Export `LD_LIBRARY_PATH`

`LD_LIBRARY_PATH` provides the Python wrapper with the path to libindy in Rust, and is necessary for the wrapper
to work properly.

```sh
export LD_LIBRARY_PATH=/<path_to_indy-sdk>/libindy/target/debug
```

## Run the Guide

```sh
python -m src.main
```
