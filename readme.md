# Legacy Mono/.NET 3.5 Random in Rust

[English](readme-en.md)

## 中文说明

这个 crate 用 Rust 实现了旧版 Mono/.NET 3.5 profile 的伪随机数生成算法，主要目标是复现《戴森球计划》（Dyson Sphere Program，DSP）中 `DotNet35Random` 的确定性随机序列。

> [!IMPORTANT]
> 这里的目标不是微软 .NET Framework 中的 `System.Random`。
>
> 旧版 Mono/DSP 在初始化时使用 `inextp = 31`，微软 Reference Source 使用 `inextp = 21`。因此，即使 seed 相同，两者生成的序列也不同。

### 兼容目标

这个项目最初移植自 DSP 中的 `DotNet35Random`。该类沿用了旧版 Mono `System.Random` 的行为，适合以下用途：

- 在 Rust 程序中复现 DSP 的世界生成、星系生成或其他依赖 seed 的逻辑；
- 读取由旧 Mono/Unity 环境产生的确定性随机序列；
- 研究旧 Mono 随机数实现与微软实现之间的差异。

这个项目不保证兼容：

- 微软 .NET Framework 2.0、3.5、4.x 的 `System.Random`；
- 现代 .NET/.NET Core 的所有 `Random` 实现与无参构造行为；
- 密码学、安全令牌、密钥、口令或任何安全敏感场景。

### 为什么是 31，而不是 21？

微软和旧 Mono 使用了非常相似的减法随机数生成器，但它们选择了不同的初始下标：

| 实现 | 初始 `inextp` | 相同 seed 的序列 |
| --- | ---: | --- |
| Microsoft Reference Source `System.Random` | 21 | 微软序列 |
| 旧版 Mono `System.Random` | 31 | Mono 序列 |
| DSP `DotNet35Random` | 31 | Mono/DSP 序列 |
| 本 crate | 31 | 目标为 Mono/DSP 序列 |

Mono 在 2004 年有意将这个值从 21 改为 31，提交说明为 “Decision to follow knuth over ms results”。`31 = 55 - 24` 对应 Knuth 算法采用的 lag。微软保留了 21，以维持既有程序的确定性序列兼容性。

相关资料：

- [Mono 将 `inextp` 从 21 改为 31 的提交](https://github.com/mono/mono/commit/b409d00f1cbade6846d02d7e8a01a677331ec6b0)
- [旧版 Mono 的 `System.Random` 实现](https://github.com/mono/mono/blob/4fe8fb883b25a429245407b52394c4762bbf421a/mcs/class/corlib/System/Random.cs)
- [微软 Reference Source 的 `System.Random`](https://github.com/microsoft/referencesource/blob/51cf7850defa8a17d815b4700b67116e3fa283c2/mscorlib/system/random.cs)
- [dotnet/runtime#23198：微软实现与标准算法的偏差](https://github.com/dotnet/runtime/issues/23198)
- [DSP 生态中的 `DotNet35Random` 实现](https://github.com/LokiSharp/DspLib/blob/fd5b73119914e0133c98eea3aca17f7a4ea89284/DspLib/Algorithms/DotNet35Random.cs)

### 用法

```rust
use dotnet35_rand_rs::DotNet35Random;

fn main() {
    let mut rng = DotNet35Random::new(1_919_810);

    assert_eq!(rng.next(), 429_045_588);
    assert_eq!(rng.next(), 1_732_108_734);
    assert_eq!(rng.next(), 970_222_201);

    let value = rng.next_double();
    println!("{value}");
}
```

如果不需要复现已有序列，可以使用当前时间创建实例：

```rust
use dotnet35_rand_rs::DotNet35Random;

let mut rng = DotNet35Random::new_now();
println!("{}", rng.next());
```

`new_now()` 使用 Unix epoch 毫秒转换成 `i32`，它并不等价于 Mono/.NET 的 `Environment.TickCount`。需要可复现结果时，请始终显式提供 seed。

### 当前 API

- `new(seed)`：使用指定的 `i32` seed 创建生成器；
- `new_now()`：使用当前系统时间生成 seed，不保证与 Mono/.NET 的无参构造器一致；
- `next()` / `next_i32()`：生成非负 `i32`；
- `next_u32()`：将同一个非负内部 sample 转换为 `u32`，不会覆盖完整 32-bit 范围；
- `next_double()` / `next_f64()`：生成 `[0, 1)` 范围内的 `f64`；
- `next_with_max(max)`：生成 `[0, max)` 范围内的值，非法参数返回 `None`；
- `next_in_range(min, max)`：生成 `[min, max)` 范围内的值，非法参数返回 `None`；
- `next_bytes(buffer)`：填充字节切片；
- `next_u8_vec(len)`：创建并填充指定长度的字节数组。

`DotNet35Const`、`new_with_const()`、`sample()`、`internal_sample()`、`get_sample_for_large_range()` 以及生成器的内部状态字段目前也是公开 API。它们会暴露或改变算法不变量，仅建议用于调试和兼容性研究；普通调用方不应直接修改内部状态。

### v1.0.4 的已知兼容性限制

当前版本仍然混合了旧 Mono/DSP 与微软 Reference Source 的部分行为。它不能被视为已经完成逐 API、逐边界条件验证的 DSP 兼容实现。

已确认的问题包括：

- `next()` 当前直接返回内部 sample；旧 Mono/DSP 会经过一次浮点乘法和整数转换，少数输出可能相差 1；
- `next_bytes()` 和 `next_u8_vec()` 当前使用 `% 255`，与 Mono/DSP 的 `Sample() * 256.0` 不一致；
- `next_in_range()` 当前采用微软的大范围处理思路，而不是旧 Mono/DSP 的 `uint diff` 行为；
- `new_now()` 使用 wall-clock Unix 时间，而不是 `Environment.TickCount`；
- 返回 `Option` 是 Rust API 选择，不等同于 C# 抛出 `ArgumentOutOfRangeException` 的行为。

基本 seed 序列已有来自 DSP/Mono 实现的回归测试，但字节生成、大范围、状态消耗顺序及所有边界 seed 仍需要更完整的差分测试。依赖严格兼容性的项目，应固定 crate 版本并自行验证实际使用的调用路径。

### 安全性

这是一个用于确定性复现的传统伪随机数生成器，不是密码学安全随机数生成器。不要用它生成密码、密钥、nonce、session token 或其他安全敏感数据。

### 关于 AI 辅助

这个项目最初在 GitHub Copilot 辅助下完成。是否使用 AI 并不能说明实现是否正确；真正重要的是明确兼容目标、保留可追溯的参考源码，并使用目标运行时产生的测试向量进行验证。本 README 记录了当前已经确认的兼容范围和限制。

## 许可证

[WTFPL](license.txt)
