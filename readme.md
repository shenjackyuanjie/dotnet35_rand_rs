# .NET 3.5 Random in Rust

一个用于模拟 .NET 3.5 Random 的 Rust 库

A Rust library for simulating .NET 3.5 Random

## 用法 Usage

```rust
use dotnet35_rand_rs::DotNet35Random;

fn main() {
    let mut rng = DotNet35Random::new(123456789);
    println!("{}", rng.next());

    let mut rng_now = DotNet35Random::new_now();
    println!("{}", rng_now.next());
}
```

## 注意 Notice

这个库就是一个 GitHub Copilot 作品
如果你在意,请不要使用,自己写一个去吧

This library is just a GitHub Copilot work
If you care, please don't use it, write one yourself
