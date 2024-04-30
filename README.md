## Machineid-rs wrapper for rust

Made this because I liked using it for Rust and cant be bothered using something else

Probably is very unsafe to use.
---
Cross build for window from linux: 

```
rustup target add x86_64-pc-windows-msvc
```
```
cargo build --release --target=x86_64-pc-windows-msvc
```

## To build C style header
Install ``cbindgen`` and run: 
```
cbindgen --lang c > machineid-rs-wrapper.h
```