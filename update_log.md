# update log

## 2.0.0

BREAKING CHANGE
- `next_in_range` 函数返回值修改为 `Option<i32>`
  - 这样就不会 panic 了
- `next_with_max` 函数返回值修改为 `Option<i32>`
  - 这样就不会 panic 了

将 `DotNet35Const` 的 `new` `new_default` 函数改为 `const fn`

## 1.0.1

感谢 【@szabgab](https://github.com/szabgab) 的 PR
在 Cargo.toml 中添加了 `repository` 字段

## 1.0.0

反正没人报 issue, 就直接 1.0.0 了
无修改

## 0.2.1

再一次的修复了与 dotnet 3.5 的行为不一致问题
(这次是我的问题, 忽略了一个 `++`)

```diff
pub fn internal_sample(&mut self) -> i32 {
+   self.inext += 1;
    if self.inext >= 56 {
        self.inext = 1;
    }
+   self.inextp += 1;
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
```

添加了新的一致性测试

## 0.2.0

使用 @DoubleUTH 提供的 Random 源码重新实现了一部分代码
参考源码已经复制在注释中

https://github.com/microsoft/referencesource/blob/51cf7850defa8a17d815b4700b67116e3fa283c2/mscorlib/system/random.cs

修复了因为使用反编译源码所导致的行为不一致

https://github.com/DoubleUTH/DSP-Seed-Finder/issues/3#issuecomment-1899478069

添加了新的 `next_f64` `next_i32` `next_u32` `next_u8_vec(len: usize)`

为 Const 和 Random 实现了 `Hash` derive

## 0.1.5

修复了 在 `0.1.4` 中不小心把 for 循环边界修改改错导致的行为不一致
感谢 @nazo-x1 的 PR

去除了 `new_now` 的 `expect`, 改为 `unwarp_or`
尽量保证不会出现 panic 问题

## 0.1.4

修复了因为算数移除 导致的 panic

## 0.1.3

修复了一个 数组溢出错误 (?)

## 0.1.2

补充了 `fn next_in_range(&mut self, min_value: i32, max_value: i32) -> i32`
原型 `public virtual int Next(int minValue, int maxValue)`

我的问题, 忘记实现它了

添加了 `pub fn new_now() -> Self`
原型 `public DotNet35Random() : this(Environment.TickCount)`

## 0.1.1

添加了更多 doc

为两个 struct 实现了 `##[derive(Clone, Copy, Debug, PartialEq, Eq)]`

## 0.1.0

Just implment it

实现了这玩意
