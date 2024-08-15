## コンパイルターゲットを追加

```bash
$ rustup target add x86_64-unknown-linux-musl
$ rustup target add aarch64-unknown-linux-musl
```

## `rustc`で直接コンパイルしてアセンブリを吐き出す

foo.rs
```Rust
#[no_mangle]
pub fn add_two(a: i32, b: i32) -> i32 {
    a + b
}
```

x86\_64でコンパイル
```bash
$ rustc --crate-type=lib -O --target=x86_64-unknown-linux-musl --emit=asm -o foo_x86_64_musl.s foo.rs
```

aarhc64でコンパイル
```bash
$ rustc --crate-type=lib -O --target=aarch64-unknown-linux-musl --emit=asm -o foo_aarch64_musl.s foo.rs
```
