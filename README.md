# rust_by_example

## Usage
```shell
rustc -o a.out 01_Hello-World.rs
./a.out

# Create a library
rustc --crate-type=lib 63_Crates-Creating-a-Library.rs
ls lib*

# Using a library
rustc 64_Crates-Using-a-Library.rs --extern rary=lib63_Crates-Creating_a_Library.rlib --edition=2018 -o a.out
./a.tou

# Using attribute
rustc 66_Attributes-Crates.rs
ls lib*

# Using attribute flag
rustc -o a.out --cfg some_condition 68_Attributes-Custom.rs
./a.tou
```

## References
- [17 Resources to Help You Learn Rust in 2021](https://serokell.io/blog/learn-rust)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/)
- [Rustの日本語ドキュメント/Japanese Docs for Rust](https://doc.rust-jp.rs/)
