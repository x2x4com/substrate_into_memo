// use std::fmt;

fn main() {
    let t1 = vec![1, u32::MAX-1];
    print_option_u32(sum_u32(&t1));

    let t2 = vec![1, u32::MAX];
    print_option_u32(sum_u32(&t2));
    // let sum = sum_u32(&t);
    // println!("Sum: {}", sum);
}

// 不知道为什么不对，出错 impl doesn't use only types from inside the current crate
// impl fmt::Display for Option<u32> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // 使用 `self.number` 来表示各个数据。
//         match f {
//             Some(t) => write!(f, "({})", t),
//             None => write!(f, "(None)")
//         }
//     }
// }

fn print_option_u32(v: Option<u32>) {
    match v {
        Some(v) => println!("sum: {}", v),
        None => println!("Value overflow!")
    };
}

fn sum_u32(v: &[u32]) -> Option<u32> {
    // 对Vec操作先转iter
    let mut total: u32 = 0;
    for i in v {
        // u32.checked_add() https://doc.rust-lang.org/std/primitive.u32.html
        total = match total.checked_add(*i) {
            Some(t) => t,
            None => return None,
        }
    }
    // let total:Option<u32> = Some(v_iter.sum());
    // total
    Some(total)
}



