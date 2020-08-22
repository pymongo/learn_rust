use itertools::Itertools;

fn main() {
    let data = ["1", "22", "4444", "666666", "999999999"];
    let batch_size = 10;
    data.iter()
        .peekable()
        .batching(|it| {
            let mut size = 0;
            let mut buf = Vec::new();
            // 功能: 对数组进行重新分组，一旦组内字符个数超过10，则开辟一个新的组
            while let Some(val) = it.next() {
                buf.push(val);
                size += val.len();

                match it.peek() {
                    None => return Some(buf),
                    Some(val) => {
                        if val.len() + size > batch_size {
                            return Some(buf);
                        }
                    }
                }
            }
            None
        })
        .map(|x| println!("{:?}", x))
        .for_each(drop);
}
