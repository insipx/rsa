
# RSA

Created for Cryptography Class


## To Run On Windows
- Download Windows Binary (Requires 64bit PC): [RSA Windows
  Binary](https://github.com/insipx/rsa/releases/download/1/Windows.64Bit.exe)
- place the binary in any folder you like
- navigate to the folder you placed the binary in with PowerShell
- Run RSA! `./rsa --help`

## To Run on Linux
- Download the Linux Binary (Requires 64bit PC): [RSA
  Linux](https://github.com/insipx/rsa/releases/download/1/GNU.Linux.64bit)
- place linux binary in a directory specified in PATH or just a directory you want to execute from
- `./rsa --help` while in same directory

## To Compile on Windows
- install Rust: [https://rustup.rs](https://rustup.rs)
- Must have VS installed with Visual C++ options
- `git clone` the repository or download and unzip this repository into any directory
- Navigate your way into the root directory of the downloaded repository
- run `cargo build --release`
- go into the directory `target/release` and copy the RSA executable to anywhere you would like to execute it
- execute the RSA executable by navigating to where you copied it. You may now use RSA, for example, `./rsa --help`


## To Compile on Linux/MacOS
- install Rust (if not already installed): [https://rustup.rs](https://rustup.rs)
- clone or download this repository
  - if you downloaded as zip, remember to unzip
- `cd` into where the repository was downloaded
- from the directory, run the command `cargo build --release`
- from the directory, run the command `cp ./target/release/rsa ~/`
- you can now run the program from you home directory like so: ./rsa --help
