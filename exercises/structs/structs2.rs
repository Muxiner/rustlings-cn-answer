// structs2.rs
// 解决所有的 TODOs 以让测试通过！
// 执行 `rustlings hint structs2` 或在观察模式下使用 `hint` 子命令来获取提示。

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////// I AM NOT DONE

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

impl Order {
    fn new(name: String) -> Self {
        Order {
            name,
            year: 2019,
            made_by_email: true,
            made_by_mobile: false,
            made_by_phone: false,
            item_number: 123,
            count: 1,
        }
    }

    fn get_new(
        name: String,
        year: u32,
        by_email: bool,
        by_mobile: bool,
        by_phone: bool,
        item_number: u32,
        count: u32,
    ) -> Self {
        Order {
            name,
            year,
            made_by_email: by_email,
            made_by_mobile: by_mobile,
            made_by_phone: by_phone,
            item_number,
            count,
        }
    }
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: 使用更新语法和上面的模板来创建你自己的订单！
        // let your_order =
        let your_order = Order::new("Hacker in Rust".to_string());
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }

    #[test]
    fn what_a_shit() {
        let order_one = Order::get_new("Hacker in Rust".to_string(), 2019, true, false, false, 123, 1);
        let order_two = Order::new("Hacker in Rust".to_string());

        assert_eq!(order_two.name, order_one.name);
    }
}
