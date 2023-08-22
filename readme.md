<header>
<img src="https://rust-lang.uz/logo.png" alt="logo" height="100" align="left">
<h1 style="display: inline">Zang</h1>

Rust language tokens translated to Uzbek

[![GitHub top language](https://img.shields.io/github/languages/top/rust-lang-uz/zang?style=flat-square&logo=github)](https://github.com/rust-lang-uz/zang)
[![Chat](https://img.shields.io/badge/Chat-grey?style=flat-square&logo=telegram)](https://t.me/rustlanguz)

</header>

## Installing 

Add this to your `Cargo.toml`:

```toml
[dependencies]
zang-macro = "0.1.0"
```

or just install it via cargo:

```shell
cargo install zang-macro
```

## Usage

You may use it partially by importing our macro or use it as a whole statically calling zang macro. Dictionary is available at: https://github.com/rust-lang-uz/zang/blob/4bca619f5ec0857cf657acb5da30487e7fc0e12e/zang-macro/src/lib.rs#L6C3-L6C3

```rust
zang_macro::zang!{
    funksiya asosiy() {
        yoziq!("Salom, Zang!");
    }
}
```
