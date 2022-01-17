# Minimal kernel and OS writted by Rust!!

This OS arch is x86_64 and host machine OS is OS X or Linux!!

## Setup Environment

* **Rust(+nightly)**
* **QEMU(x86_64)**


```sh
cargo install cargo-binutils bootimage

rustup toolchain install nightly

(rustup default nightly) # default stable -> nightly 

rustup component add rust-src llvm-tools-preview
```


## Run

```sh
git clone git@github.com:Ryusei-0407/fledge-os.git

cd fledge-os

cargo +nightly run

(make run) # You have gnumake
```

