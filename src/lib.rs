//! 一个模拟 .NET 3.5 的 Random 类的库
//!
//! A library for simulating the .NET 3.5 Random class
//!
//! 用法 Usage:
//! ```rust
//! use dotnet35_rand_rs::DotNet35Random;
//!
//! fn main() {
//!   let mut rand = DotNet35Random::new(0);
//!   println!("{}", rand.next());
//! }
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DotNet35Const {
    pub mz: i32,
    pub mbig: i32,
    pub mseed: i32,
}

impl DotNet35Const {
    /// "万一你真需要修改常量呢?"
    ///
    /// "What if you really need to modify the constants?"
    pub fn new(mz: i32, mbig: i32, mseed: i32) -> Self {
        Self { mz, mbig, mseed }
    }
}

impl Default for DotNet35Const {
    /// 默认值
    ///
    /// default value
    fn default() -> Self {
        Self {
            mz: 0,
            mbig: 2147483647,
            mseed: 161803398,
        }
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
/// fn main() {
///    let mut rand = DotNet35Random::new(0);
///    println!("{}", rand.next());
/// }
/// ```
///
/// 参考源码 Reference source code:
///
/// ```csharp
/// using System;
/// using System.Runtime.InteropServices;
///
/// // Token: 0x02000040 RID: 64
/// [ComVisible(true)]
/// [Serializable]
/// public class DotNet35Random
/// {
/// 	// Token: 0x0600021A RID: 538 RVA: 0x00015C20 File Offset: 0x00013E20
/// 	public DotNet35Random() : this(Environment.TickCount)
/// 	{
/// 	}
///
/// 	// Token: 0x0600021B RID: 539 RVA: 0x00015C30 File Offset: 0x00013E30
/// 	public DotNet35Random(int Seed)
/// 	{
/// 		int num = 161803398 - Math.Abs(Seed);
/// 		this.SeedArray[55] = num;
/// 		int num2 = 1;
/// 		for (int i = 1; i < 55; i++)
/// 		{
/// 			int num3 = 21 * i % 55;
/// 			this.SeedArray[num3] = num2;
/// 			num2 = num - num2;
/// 			if (num2 < 0)
/// 			{
/// 				num2 += int.MaxValue;
/// 			}
/// 			num = this.SeedArray[num3];
/// 		}
/// 		for (int j = 1; j < 5; j++)
/// 		{
/// 			for (int k = 1; k < 56; k++)
/// 			{
/// 				this.SeedArray[k] -= this.SeedArray[1 + (k + 30) % 55];
/// 				if (this.SeedArray[k] < 0)
/// 				{
/// 					this.SeedArray[k] += int.MaxValue;
/// 				}
/// 			}
/// 		}
/// 		this.inext = 0;
/// 		this.inextp = 31;
/// 	}
///
/// 	// Token: 0x0600021C RID: 540 RVA: 0x00015D14 File Offset: 0x00013F14
/// 	protected virtual double Sample()
/// 	{
/// 		int num = this.inext + 1;
/// 		this.inext = num;
/// 		if (num >= 56)
/// 		{
/// 			this.inext = 1;
/// 		}
/// 		num = this.inextp + 1;
/// 		this.inextp = num;
/// 		if (num >= 56)
/// 		{
/// 			this.inextp = 1;
/// 		}
/// 		int num2 = this.SeedArray[this.inext] - this.SeedArray[this.inextp];
/// 		if (num2 < 0)
/// 		{
/// 			num2 += int.MaxValue;
/// 		}
/// 		this.SeedArray[this.inext] = num2;
/// 		return (double)num2 * 4.656612875245797E-10;
/// 	}
///
/// 	// Token: 0x0600021D RID: 541 RVA: 0x00015D9B File Offset: 0x00013F9B
/// 	public virtual int Next()
/// 	{
/// 		return (int)(this.Sample() * 2147483647.0);
/// 	}
///
/// 	// Token: 0x0600021E RID: 542 RVA: 0x00015DAE File Offset: 0x00013FAE
/// 	public virtual int Next(int maxValue)
/// 	{
/// 		if (maxValue < 0)
/// 		{
/// 			throw new ArgumentOutOfRangeException(DotNet35Locale.GetText("Max value is less than min value."));
/// 		}
/// 		return (int)(this.Sample() * (double)maxValue);
/// 	}
///
/// 	// Token: 0x0600021F RID: 543 RVA: 0x00015DD0 File Offset: 0x00013FD0
/// 	public virtual int Next(int minValue, int maxValue)
/// 	{
/// 		if (minValue > maxValue)
/// 		{
/// 			throw new ArgumentOutOfRangeException(DotNet35Locale.GetText("Min value is greater than max value."));
/// 		}
/// 		uint num = (uint)(maxValue - minValue);
/// 		if (num <= 1U)
/// 		{
/// 			return minValue;
/// 		}
/// 		return (int)((ulong)((uint)(this.Sample() * num)) + (ulong)((long)minValue));
/// 	}
///
/// 	// Token: 0x06000220 RID: 544 RVA: 0x00015E0C File Offset: 0x0001400C
/// 	public virtual void NextBytes(byte[] buffer)
/// 	{
/// 		if (buffer == null)
/// 		{
/// 			throw new ArgumentNullException("buffer");
/// 		}
/// 		for (int i = 0; i < buffer.Length; i++)
/// 		{
/// 			buffer[i] = (byte)(this.Sample() * 256.0);
/// 		}
/// 	}
///
/// 	// Token: 0x06000221 RID: 545 RVA: 0x00015E49 File Offset: 0x00014049
/// 	public virtual double NextDouble()
/// 	{
/// 		return this.Sample();
/// 	}
///
/// 	// Token: 0x04000302 RID: 770
/// 	private const int MBIG = 2147483647;
///
/// 	// Token: 0x04000303 RID: 771
/// 	private const int MSEED = 161803398;
///
/// 	// Token: 0x04000304 RID: 772
/// 	private const int MZ = 0;
///
/// 	// Token: 0x04000305 RID: 773
/// 	private int inext;
///
/// 	// Token: 0x04000306 RID: 774
/// 	private int inextp;
///
/// 	// Token: 0x04000307 RID: 775
/// 	private int[] SeedArray = new int[56];
/// }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DotNet35Random {
    /// int next
    pub inext: usize,
    /// int nextp
    pub inextp: usize,
    /// int\[56\] SeedArray
    pub seed_array: [i32; 56],
    /// consts
    pub consts: DotNet35Const,
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
            .expect("Time went backwards");
        let tick_count = since_the_epoch.as_millis() as i32;
        DotNet35Random::new_with_const(tick_count, consts)
    }

    /// "万一你真需要修改常量呢?"
    ///
    /// "What if you really need to modify the constants?"
    pub fn new_with_const(seed: i32, consts: DotNet35Const) -> Self {
        let mut seed_array = [0; 56];
        let mut num = consts.mseed - seed.abs();
        seed_array[55] = num;
        let mut num2 = 1;
        for i in 1..55 {
            let num3 = 21 * i % 55;
            seed_array[num3] = num2;
            num2 = num.wrapping_sub(num2);
            if num2 < consts.mz {
                // 可能导致算数溢出, 使用 wrapping_add
                num2 = num2.wrapping_add(consts.mbig);
            }
            num = seed_array[num3];
        }
        for _ in 1..5 {
            for k in 1..55 {
                // 可能导致算数溢出, 使用 wrapping_sub 和 wrapping_add
                seed_array[k] = seed_array[k].wrapping_sub(seed_array[1 + (k + 30) % 55]);
                if seed_array[k] < 0 {
                    seed_array[k] = seed_array[k].wrapping_add(i32::MAX);
                }
            }
        }
        Self {
            inext: 0,
            inextp: 31,
            seed_array,
            consts,
        }
    }

    /// `protected virtual double Sample()`
    ///
    /// 原始的 Sample 方法
    /// original Sample method
    pub fn sample(&mut self) -> f64 {
        let num = self.inext + 1;
        self.inext = num;
        if num >= 56 {
            self.inext = 1;
        }
        let num = self.inextp + 1;
        self.inextp = num;
        if num >= 56 {
            self.inextp = 1;
        }
        let num2 = self.seed_array[self.inext] - self.seed_array[self.inextp];
        (if num2 < self.consts.mz {
            num2 + self.consts.mbig
        } else {
            num2
        }) as f64
            * 4.656612875245797E-10
    }

    /// `public virtual int Next()`
    ///
    /// 默认的 Next 方法
    /// default Next method
    pub fn next(&mut self) -> i32 {
        (self.sample() * 2147483647.0) as i32
    }

    /// `public virtual int Next(int maxValue)`
    ///
    /// 限制最大值的 Next 方法
    /// Next method with max value
    pub fn next_with_max(&mut self, max_value: i32) -> i32 {
        if max_value < self.consts.mz {
            panic!("Max value is less than min value.");
        }
        (self.sample() * max_value as f64) as i32
    }

    /// `public virtual int Next(int minValue, int maxValue)`
    ///
    /// 限制最大值和最小值的 Next 方法
    /// Next method with max value and min value
    pub fn next_in_range(&mut self, min_value: i32, max_value: i32) -> i32 {
        if min_value > max_value {
            panic!("Min value is greater than max value.");
        }
        let num = max_value - min_value;
        if num <= 1 {
            return min_value;
        }
        (self.sample() * num as f64) as i32 + min_value
    }

    /// `public virtual void NextBytes(byte[] buffer)`
    ///
    /// 返回随机字节的 NextBytes 方法
    /// NextBytes method with random bytes
    pub fn next_bytes(&mut self, buffer: &mut [u8]) {
        for i in 0..buffer.len() {
            buffer[i] = (self.sample() * 256.0) as u8;
        }
    }

    /// `public virtual double NextDouble()`
    ///
    /// 返回随机浮点数的 NextDouble 方法
    /// NextDouble method with random double
    pub fn next_double(&mut self) -> f64 {
        self.sample()
    }
}

#[test]
fn create() {
    let mut rand = DotNet35Random::new(0);
    assert_eq!(rand.next(), 1976681210);
    assert_eq!(rand.next(), 551155468);
}

#[test]
fn create_now() {
    let mut rand = DotNet35Random::new_now();
    for _ in 0..100 {
        assert!(rand.next_with_max(1000) < 1000);
        let next = rand.next_in_range(1000, 2000);
        assert!(next >= 1000 && next < 2000);
    }
}
