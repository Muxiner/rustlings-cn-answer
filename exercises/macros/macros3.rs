// macros3.rs
// 让我可以编译，在不将宏移出模块的前提下！
// 执行 `rustlings hint macros3` 或在观察模式下使用 `hint` 子命令来获取提示。

// // I AM NOT DONE

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

// use macros::my_macro;

fn main() {
    ::my_macro!();
}
