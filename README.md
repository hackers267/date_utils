# DateUtils 

The date_utils is a package with a set of functions that operate on the date.**Date-Utils** is a powerful date and time manipulation library for Rust that relies on the well-known `chrono` library. Inspired by JavaScript's `date-fns`, this library aims to provide Rust developers with a rich set of features and an intuitive API.

Key features include:
1. Dependency on `chrono` for comprehensive and precise date and time handling capabilities.
2. A wide range of date and time operations such as addition, subtraction, formatting, parsing, comparison, adjustments, and utility functions.
3. High performance through optimized internal implementation and support from `chrono`.
4. Easy to use with a clear and consistent API design for rapid onboarding and efficient development.
5. Compatibility with ISO 8601 standards and support for time zone-related operations.

To get started, add the dependencies to your `Cargo.toml` file:

```toml
[dependencies]
date-utils = "0.1"
chrono = "0.4"
```

Then, import and use the library in your Rust code:

```rust
use date_utils::{DateUtils, FormatItem};
use chrono::prelude::*;

fn main() {
    let now: DateTime<Utc> = Utc::now();
    let formatted_date = DateUtils::format(now, &[FormatItem::Year, FormatItem::Month, FormatItem::Day]);
    println!("Today's date is: {}", formatted_date);
}
```

Detailed API documentation and usage examples can be found in the project's [official documentation](https://date-utils.github.io/docs/).

Contributions and feedback from the community are welcome. If you encounter any issues or have suggestions for new features, please submit an issue or pull request.

The library is released under the GPL-3.0 License. For more information, refer to the LICENSE file.

Let's enjoy seamless date and time manipulation experiences in Rust! 🎉

