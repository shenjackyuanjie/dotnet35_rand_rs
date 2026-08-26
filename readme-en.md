# Legacy Mono/.NET 3.5 Random in Rust

[中文](readme.md)

This crate implements the pseudo-random number generator used by the legacy Mono/.NET 3.5 profile. Its primary goal is to reproduce deterministic `DotNet35Random` sequences used by Dyson Sphere Program (DSP).

> [!IMPORTANT]
> The target is not `System.Random` from Microsoft .NET Framework.
>
> Legacy Mono and DSP initialize the generator with `inextp = 31`, while the Microsoft Reference Source uses `inextp = 21`. The two implementations therefore produce different sequences from the same seed.

## Compatibility target

This project was originally ported from DSP's `DotNet35Random`, which preserves the behavior of the legacy Mono `System.Random` implementation. It is intended for:

- reproducing DSP world generation, galaxy generation, and other seed-dependent logic in Rust;
- consuming deterministic sequences created by legacy Mono/Unity environments;
- studying the behavioral differences between legacy Mono and Microsoft implementations.

This project does not promise compatibility with:

- `System.Random` from Microsoft .NET Framework 2.0, 3.5, or 4.x;
- every `Random` implementation or parameterless-constructor behavior in modern .NET/.NET Core;
- cryptography, security tokens, keys, passwords, or any security-sensitive use case.

## Why 31 instead of 21?

Microsoft and legacy Mono used closely related subtractive random number generators, but selected different initial indices:

| Implementation | Initial `inextp` | Sequence for the same seed |
| --- | ---: | --- |
| Microsoft Reference Source `System.Random` | 21 | Microsoft sequence |
| Legacy Mono `System.Random` | 31 | Mono sequence |
| DSP `DotNet35Random` | 31 | Mono/DSP sequence |
| This crate | 31 | Targets the Mono/DSP sequence |

Mono intentionally changed this value from 21 to 31 in 2004 in a commit titled “Decision to follow knuth over ms results.” The value `31 = 55 - 24` corresponds to the lag used by Knuth's algorithm. Microsoft retained 21 to preserve deterministic compatibility for existing applications.

References:

- [Mono commit changing `inextp` from 21 to 31](https://github.com/mono/mono/commit/b409d00f1cbade6846d02d7e8a01a677331ec6b0)
- [Legacy Mono `System.Random` implementation](https://github.com/mono/mono/blob/4fe8fb883b25a429245407b52394c4762bbf421a/mcs/class/corlib/System/Random.cs)
- [Microsoft Reference Source `System.Random`](https://github.com/microsoft/referencesource/blob/51cf7850defa8a17d815b4700b67116e3fa283c2/mscorlib/system/random.cs)
- [dotnet/runtime#23198: deviation from the standard algorithm](https://github.com/dotnet/runtime/issues/23198)
- [`DotNet35Random` implementation from the DSP ecosystem](https://github.com/LokiSharp/DspLib/blob/fd5b73119914e0133c98eea3aca17f7a4ea89284/DspLib/Algorithms/DotNet35Random.cs)

## Usage

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

If sequence reproduction is not required, an instance can be seeded from the current time:

```rust
use dotnet35_rand_rs::DotNet35Random;

let mut rng = DotNet35Random::new_now();
println!("{}", rng.next());
```

`new_now()` converts Unix-epoch milliseconds to an `i32`; it is not equivalent to Mono/.NET `Environment.TickCount`. Always provide an explicit seed when reproducibility matters.

## Current API

- `new(seed)`: creates a generator from a specified `i32` seed;
- `new_now()`: derives a seed from the current system time and does not match the Mono/.NET parameterless constructor exactly;
- `next()` / `next_i32()`: generates a non-negative `i32`;
- `next_u32()`: casts the same non-negative internal sample to `u32` and does not cover the full 32-bit range;
- `next_double()` / `next_f64()`: generates an `f64` in `[0, 1)`;
- `next_with_max(max)`: generates a value in `[0, max)` and returns `None` for invalid input;
- `next_in_range(min, max)`: generates a value in `[min, max)` and returns `None` for invalid input;
- `next_bytes(buffer)`: fills a byte slice;
- `next_u8_vec(len)`: allocates and fills a byte vector of the requested length.

`DotNet35Const`, `new_with_const()`, `sample()`, `internal_sample()`, `get_sample_for_large_range()`, and the generator's internal state fields are also public in the current API. They expose or can violate algorithm invariants and are intended only for debugging and compatibility research; ordinary callers should not mutate the internal state directly.

## Known compatibility limitations in v1.0.4

The current release still mixes behavior from legacy Mono/DSP and the Microsoft Reference Source. It should not yet be treated as a fully verified, API-for-API DSP compatibility implementation.

Confirmed issues include:

- `next()` currently returns the internal sample directly; legacy Mono/DSP performs a floating-point multiplication and integer conversion, which can make rare outputs differ by 1;
- `next_bytes()` and `next_u8_vec()` currently use `% 255`, which does not match Mono/DSP's `Sample() * 256.0` behavior;
- `next_in_range()` currently follows Microsoft's large-range approach rather than legacy Mono/DSP's `uint diff` behavior;
- `new_now()` uses Unix wall-clock time instead of `Environment.TickCount`;
- returning `Option` is a Rust API choice and does not reproduce C#'s `ArgumentOutOfRangeException` behavior.

Basic seeded sequences have regression vectors taken from the DSP/Mono implementation. Byte generation, large ranges, state-consumption order, and all boundary seeds still require more complete differential testing. Projects requiring strict compatibility should pin the crate version and verify the exact call paths they use.

## Security

This is a legacy pseudo-random number generator for deterministic reproduction, not a cryptographically secure random number generator. Do not use it for passwords, keys, nonces, session tokens, or other security-sensitive data.

## AI assistance

This project was initially created with assistance from GitHub Copilot. The use of AI does not establish whether an implementation is correct; what matters is a clearly defined compatibility target, traceable source material, and test vectors produced by the target runtime. This README documents the compatibility scope and limitations currently known.

## License

[WTFPL](license.txt)
