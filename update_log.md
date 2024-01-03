# update log

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
