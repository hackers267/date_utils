# DateUtils

date_utils 是一个操作date的工具集。# Date-Utils: 强大的日期和时间处理库 for Rust

**Date-Utils** 是一个基于Rust的日期和时间处理库，依赖于知名的`chrono`库，旨在提供便捷、灵活和高效的日期与时间操作。灵感来源于JavaScript中的date-fns，我们致力于为Rust开发者带来同样丰富的功能和直观的API。

## 主要特性

1. **依赖于chrono**：借助`chrono`的强大功能，Date-Utils提供了更全面和精确的日期时间处理能力。

2. **全面的日期时间操作**：包括日期加减、格式化、解析、比较、调整以及各种实用函数，满足你的各种需求。

3. **高性能**：通过优化的内部实现和`chrono`的支撑，Date-Utils在处理大量日期时间数据时能保持出色的性能。

4. **易于使用**：提供清晰、一致的API设计，让你能够快速上手并高效地进行开发。

5. **兼容性**：遵循ISO 8601标准，并支持处理时区相关的操作。

## 快速开始

要在你的Cargo.toml文件中添加Date-Utils依赖：

```toml
[dependencies]
date-utils = "0.1"
chrono = "0.4"
```

然后在你的Rust代码中引入并使用库：

```rust
use date_utils::HourHelper;
use chrono::prelude::*;

fn main() {

    fn gen_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Option<NaiveDateTime> {
        NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
    }
    let one = gen_time(2000, 1, 1, 0, 0, 0).unwrap();
    let actual = gen_time(2000, 1, 1, 6, 0, 0).unwrap();
    let diff = actual.diff_hours(&one);
    println!("The diff is {}",diff);
}
```

## 文档与示例

详细的API文档和使用示例可以在我们的[官方文档](https://date-utils.github.io/docs/)中找到。

## 参与贡献

我们非常欢迎社区的贡献和反馈！如果你发现任何问题或者有新的功能建议，欢迎提交 issue 或者 pull request。

## 许可证

Date-Utils 使用 GPL-3.0 许可证发布。有关详细信息，请参阅LICENSE文件。

让我们一起在Rust中享受无缝的日期和时间处理体验 🎉
