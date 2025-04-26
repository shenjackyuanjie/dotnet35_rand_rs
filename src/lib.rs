//! 一个模拟 .NET 3.5 的 Random 类的库
//!
//! A library for simulating the .NET 3.5 Random class
//!
//! 用法 Usage:
//! ```rust
//! use dotnet35_rand_rs::DotNet35Random;
//!
//! let mut rand = DotNet35Random::new(0);
//! println!("{}", rand.next());
//!
//! ```
//!
//! by shenjackyuanjie

use std::default::Default;
use std::time::{SystemTime, UNIX_EPOCH};

/// .NET 3.5 的 Random 类的常量
/// Constants of .NET 3.5 Random class
///
/// "万一你真需要修改常量呢?"
///
/// "What if you really need to modify the constants?"
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DotNet35Const {
    pub mz: i32,
    pub mbig: i32,
    pub mseed: i32,
}

impl DotNet35Const {
    /// 默认的种子
    pub const DEFAULT_MSEED: i32 = 161803398;

    /// "万一你真需要修改常量呢?"
    ///
    /// "What if you really need to modify the constants?"
    pub const fn new(mz: i32, mbig: i32, mseed: i32) -> Self {
        Self { mz, mbig, mseed }
    }

    pub const fn new_default() -> Self {
        Self {
            mz: 0,
            mbig: i32::MAX,
            mseed: Self::DEFAULT_MSEED,
        }
    }
}

impl Default for DotNet35Const {
    /// 默认值
    ///
    /// default value
    fn default() -> Self {
        Self::new_default()
    }
}

/// 用于模拟 .NET 3.5 的 Random 类
/// A struct for simulating the .NET 3.5 Random class
///
/// 所有的方法都是公开的, 以防万一 (点名批评某个模拟Java random的库)
///
/// All methods are public, just in case you need it
///
/// 用法 Usage:
/// ```rust
/// use dotnet35_rand_rs::DotNet35Random;
///
/// let mut rand = DotNet35Random::new(0);
/// println!("{}", rand.next());
/// ```
///
/// 参考源码 Reference source code:
/// 感谢 @DoubleUTH 提供的微软源码的链接
/// Thanks to @DoubleUTH for providing the link to the Microsoft source code
///
/// 请注意, 这里的源码进行了二次格式化, 看着好看一些, 其余没有修改
/// Notice: The source code here has been formatted twice to make it look better, and the rest has not been modified
/// https://github.com/microsoft/referencesource/blob/51cf7850defa8a17d815b4700b67116e3fa283c2/mscorlib/system/random.cs
/// ```csharp
/// // ==++==
/// //
/// //   Copyright (c) Microsoft Corporation.  All rights reserved.
/// //
/// // ==--==
/// /*============================================================
/// **
/// ** Class:  Random
/// **
/// **
/// ** Purpose: A random number generator.
/// **
/// **
/// ===========================================================*/
/// namespace System
/// {
///     using System;
///     using System.Runtime;
///     using System.Runtime.CompilerServices;
///     using System.Globalization;
///     using System.Diagnostics.Contracts;
///     [System.Runtime.InteropServices.ComVisible(true)]
///     [Serializable]
///     public class Random
///     {
///         //
///         // Private Constants
///         //
///         private const int MBIG = Int32.MaxValue;
///         private const int MSEED = 161803398;
///         private const int MZ = 0;
///         //
///         // Member Variables
///         //
///         private int inext;
///         private int inextp;
///         private int[] SeedArray = new int[56];
///         //
///         // Constructors
///         //
///
///         public Random()
///           : this(Environment.TickCount)
///         {
///         }
///         public Random(int Seed)
///         {
///             int ii;
///             int mj, mk;
///
///             //Initialize our Seed array.
///             //This algorithm comes from Numerical Recipes in C (2nd Ed.)
///             int subtraction = (Seed == Int32.MinValue) ? Int32.MaxValue : Math.Abs(Seed);
///             mj = MSEED - subtraction;
///             SeedArray[55] = mj;
///             mk = 1;
///             for (int i = 1; i < 55; i++)
///             {  //Apparently the range [1..55] is special (Knuth) and so we're wasting the 0'th position.
///                 ii = (21 * i) % 55;
///                 SeedArray[ii] = mk;
///                 mk = mj - mk;
///                 if (mk < 0) mk += MBIG;
///                 mj = SeedArray[ii];
///             }
///             for (int k = 1; k < 5; k++)
///             {
///                 for (int i = 1; i < 56; i++)
///                 {
///                     SeedArray[i] -= SeedArray[1 + (i + 30) % 55];
///                     if (SeedArray[i] < 0) SeedArray[i] += MBIG;
///                 }
///             }
///             inext = 0;
///             inextp = 21;
///             Seed = 1;
///         }
///         //
///         // Package Private Methods
///         //
///         /*====================================Sample====================================
///         **Action: Return a new random number [0..1) and reSeed the Seed array.
///         **Returns: A double [0..1)
///         **Arguments: None
///         **Exceptions: None
///         ==============================================================================*/
///         protected virtual double Sample()
///         {
///             //Including this division at the end gives us significantly improved
///             //random number distribution.
///             return (InternalSample() * (1.0 / MBIG));
///         }
///         private int InternalSample()
///         {
///             int retVal;
///             int locINext = inext;
///             int locINextp = inextp;
///
///             if (++locINext >= 56) locINext = 1;
///             if (++locINextp >= 56) locINextp = 1;
///
///             retVal = SeedArray[locINext] - SeedArray[locINextp];
///
///             if (retVal == MBIG) retVal--;
///             if (retVal < 0) retVal += MBIG;
///
///             SeedArray[locINext] = retVal;
///
///             inext = locINext;
///             inextp = locINextp;
///
///             return retVal;
///         }
///         //
///         // Public Instance Methods
///         //
///         /*=====================================Next=====================================
///         **Returns: An int [0..Int32.MaxValue)
///         **Arguments: None
///         **Exceptions: None.
///         ==============================================================================*/
///         public virtual int Next()
///         {
///             return InternalSample();
///         }
///         private double GetSampleForLargeRange()
///         {
///             // The distribution of double value returned by Sample
///             // is not distributed well enough for a large range.
///             // If we use Sample for a range [Int32.MinValue..Int32.MaxValue)
///             // We will end up getting even numbers only.
///
///             int result = InternalSample();
///             // Note we can't use addition here. The distribution will be bad if we do that.
///             bool negative = (InternalSample() % 2 == 0) ? true : false;  // decide the sign based on second sample
///             if (negative)
///             {
///                 result = -result;
///             }
///             double d = result;
///             d += (Int32.MaxValue - 1); // get a number in range [0 .. 2 * Int32MaxValue - 1)
///             d /= 2 * (uint)Int32.MaxValue - 1;
///             return d;
///         }
///         /*=====================================Next=====================================
///         **Returns: An int [minvalue..maxvalue)
///         **Arguments: minValue -- the least legal value for the Random number.
///         **           maxValue -- One greater than the greatest legal return value.
///         **Exceptions: None.
///         ==============================================================================*/
///         public virtual int Next(int minValue, int maxValue)
///         {
///             if (minValue > maxValue)
///             {
///                 throw new ArgumentOutOfRangeException("minValue", Environment.GetResourceString("Argument_MinMaxValue", "minValue", "maxValue"));
///             }
///             Contract.EndContractBlock();
///
///             long range = (long)maxValue - minValue;
///             if (range <= (long)Int32.MaxValue)
///             {
///                 return ((int)(Sample() * range) + minValue);
///             }
///             else
///             {
///                 return (int)((long)(GetSampleForLargeRange() * range) + minValue);
///             }
///         }
///         /*=====================================Next=====================================
///         **Returns: An int [0..maxValue)
///         **Arguments: maxValue -- One more than the greatest legal return value.
///         **Exceptions: None.
///         ==============================================================================*/
///         public virtual int Next(int maxValue)
///         {
///             if (maxValue < 0)
///             {
///                 throw new ArgumentOutOfRangeException("maxValue", Environment.GetResourceString("ArgumentOutOfRange_MustBePositive", "maxValue"));
///             }
///             Contract.EndContractBlock();
///             return (int)(Sample() * maxValue);
///         }
///         /*=====================================Next=====================================
///         **Returns: A double [0..1)
///         **Arguments: None
///         **Exceptions: None
///         ==============================================================================*/
///         public virtual double NextDouble()
///         {
///             return Sample();
///         }
///         /*==================================NextBytes===================================
///         **Action:  Fills the byte array with random bytes [0..0x7f].  The entire array is filled.
///         **Returns:Void
///         **Arugments:  buffer -- the array to be filled.
///         **Exceptions: None
///         ==============================================================================*/
///         public virtual void NextBytes(byte[] buffer)
///         {
///             if (buffer == null) throw new ArgumentNullException("buffer");
///             Contract.EndContractBlock();
///             for (int i = 0; i < buffer.Length; i++)
///             {
///                 buffer[i] = (byte)(InternalSample() % (Byte.MaxValue + 1));
///             }
///         }
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DotNet35Random {
    /// private int inext;
    pub inext: usize,
    /// private int inextp;
    pub inextp: usize,
    /// private int[] SeedArray = new int[56];
    pub seed_array: [i32; 56],
    /// consts
    pub consts: DotNet35Const,
    /// seed
    pub seed: i32,
}

impl DotNet35Random {
    /// 默认的构造函数
    ///
    /// default constructor
    pub fn new(seed: i32) -> Self {
        let consts = DotNet35Const::default();
        DotNet35Random::new_with_const(seed, consts)
    }

    /// `public DotNet35Random() : this(Environment.TickCount)`
    ///
    /// 用当前时间作为种子的构造函数
    /// Constructor with current time as seed
    pub fn new_now() -> Self {
        let consts = DotNet35Const::default();
        // Environment.TickCount

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .unwrap_or(std::time::Duration::new(0, 0));
        let tick_count = since_the_epoch.as_millis() as i32;
        DotNet35Random::new_with_const(tick_count, consts)
    }

    /// "万一你真需要修改常量呢?"
    ///
    /// "What if you really need to modify the constants?"
    pub fn new_with_const(seed: i32, consts: DotNet35Const) -> Self {
        let mut seed_array = [0; 56];
        let subtraction = if seed == i32::MIN {
            i32::MAX
        } else {
            seed.abs()
        };
        let mut mj: i32 = consts.mseed - subtraction;
        seed_array[55] = mj;
        let mut mk = 1;

        for i in 1..55 {
            let ii = 21 * i % 55;
            seed_array[ii] = mk;

            // overflow warning! use wrapping_sub
            mk = mj.wrapping_sub(mk);
            if mk < 0 {
                // 可能导致算数溢出, 使用 wrapping_add
                mk = mk.wrapping_add(consts.mbig);
            }
            mj = seed_array[ii];
        }
        for _ in 1..5 {
            for i in 1..56 {
                // 可能导致算数溢出, 使用 wrapping_sub 和 wrapping_add
                seed_array[i] = seed_array[i].wrapping_sub(seed_array[1 + (i + 30) % 55]);
                if seed_array[i] < 0 {
                    seed_array[i] = seed_array[i].wrapping_add(consts.mbig);
                }
            }
        }
        Self {
            inext: 0,
            inextp: 31,
            seed_array,
            consts,
            seed,
        }
    }

    /// `private double GetSampleForLargeRange()`
    ///
    /// 原始的 GetSampleForLargeRange 方法
    /// original GetSampleForLargeRange method
    ///
    /// 源码注释翻译:
    /// Sample 返回的 double 值的分布对于大范围来说不够好
    /// 如果我们使用 Sample 来生成 [Int32.MinValue..Int32.MaxValue) 的范围
    /// 我们将只会得到偶数
    ///
    /// Source code comment:
    /// The distribution of double value returned by Sample
    /// is not distributed well enough for a large range.
    /// if we use Sample for a range [Int32.MinValue..Int32.MaxValue)
    /// we will end up getting even numbers only.
    pub fn get_sample_for_large_range(&mut self) -> f64 {
        let result = self.internal_sample();
        // 注意, 我们不能使用加法. 如果我们这么做, 分布将会很糟糕
        let negative = self.internal_sample() % 2 == 0;
        // 根据第二个样本决定符号
        let result = if negative { -result } else { result };
        let mut d = result as f64;
        d += (i32::MAX - 1) as f64;
        // 获取一个范围在 [0 .. 2 * Int32MaxValue - 1) 的数字
        // 我专门去查了一下 C# 的运算符优先级来写的这一行代码
        // I specially checked the operator priority of C# to write this line of code
        d /= ((i32::MAX as u32 * 2) - 1) as f64;
        d
    }

    /// `protected virtual double Sample()`
    ///
    /// 原始的 Sample 方法
    /// original Sample method
    pub fn sample(&mut self) -> f64 {
        self.internal_sample() as f64 * (1.0 / self.consts.mbig as f64)
    }

    /// `private int InternalSample()`
    ///
    /// 原始的 InternalSample 方法
    /// original InternalSample method
    pub fn internal_sample(&mut self) -> i32 {
        self.inext += 1;
        if self.inext >= 56 {
            self.inext = 1;
        }
        self.inextp += 1;
        if self.inextp >= 56 {
            self.inextp = 1;
        }
        let mut ret_val = self.seed_array[self.inext] - self.seed_array[self.inextp];
        if ret_val == self.consts.mbig {
            ret_val -= 1;
        }
        if ret_val < 0 {
            ret_val += i32::MAX;
        }
        self.seed_array[self.inext] = ret_val;
        ret_val
    }

    /// `public virtual int Next()`
    ///
    /// 默认的 Next 方法
    /// default Next method
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        self.internal_sample()
    }

    /// `public virtual int Next(int maxValue)`
    ///
    /// 限制最大值的 Next 方法
    /// Next method with max value
    pub fn next_with_max(&mut self, max_value: i32) -> i32 {
        if max_value < self.consts.mz {
            panic!("Max value is less than min value, not {}", max_value);
        }
        (self.sample() * max_value as f64) as i32
    }

    /// `public virtual int Next(int minValue, int maxValue)`
    ///
    /// 限制最大值和最小值的 Next 方法
    /// Next method with max value and min value
    pub fn next_in_range(&mut self, min_value: i32, max_value: i32) -> Option<i32> {
        if min_value > max_value {
            return None;
        }
        let range = max_value as i64 - min_value as i64;
        if range <= i32::MAX as i64 {
            Some((self.sample() * range as f64) as i32 + min_value)
        } else {
            Some((self.get_sample_for_large_range() * range as f64) as i32 + min_value)
        }
    }

    /// `public virtual void NextBytes(byte[] buffer)`
    ///
    /// 返回随机字节的 NextBytes 方法
    /// NextBytes method with random bytes
    pub fn next_bytes(&mut self, buffer: &mut [u8]) {
        (0..buffer.len()).for_each(|i| {
            buffer[i] = (self.internal_sample() % u8::MAX as i32) as u8;
        });
    }

    /// `public virtual double NextDouble()`
    ///
    /// 返回随机浮点数的 NextDouble 方法
    /// NextDouble method with random double
    pub fn next_double(&mut self) -> f64 {
        self.sample()
    }

    /// 更 rusty 一些的 next_f64 方法
    /// more rusty next_f64 method
    pub fn next_f64(&mut self) -> f64 {
        self.sample()
    }

    /// 更 rusty 一些的 next_i32 方法
    /// more rusty next_i32 method
    pub fn next_i32(&mut self) -> i32 {
        self.internal_sample()
    }

    /// 更 rusty 一些的 next_u32 方法
    /// more rusty next_u32 method
    pub fn next_u32(&mut self) -> u32 {
        self.internal_sample() as u32
    }

    pub fn next_u8_vec(&mut self, len: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push((self.internal_sample() % u8::MAX as i32) as u8);
        }
        vec
    }
}

#[test]
fn create() {
    let mut rand = DotNet35Random::new(0);
    assert_eq!(rand.next(), 1976681210);
    assert_eq!(rand.next(), 551155468);
}

#[test]
fn verify() {
    let mut rand = DotNet35Random::new(1919810);
    assert_eq!(rand.next(), 429045588);
    assert_eq!(rand.next(), 1732108734);
    assert_eq!(rand.next(), 970222201);
    assert_eq!(rand.next(), 369077841);
    assert_eq!(rand.next_double(), 0.11859490215712921);
    assert_eq!(rand.next(), 298877408);
}

#[test]
fn create_now() {
    let mut rand = DotNet35Random::new_now();
    for _ in 0..100 {
        assert!(rand.next_with_max(1000) < 1000);
        let next = rand.next_in_range(1000, 2000);
        assert!((1000..2000).contains(&next));
    }
}
