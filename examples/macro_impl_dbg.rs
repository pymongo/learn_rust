macro_rules! my_dbg {
    ($expr:expr) => {
        println!(
            "[{}:{}] {} = {:#?}",
            file!(),
            line!(),
            stringify!($expr),
            $expr
        );
    };
}

fn main() {
    my_dbg!(1 + 1);
    dbg!(1 + 1);
    my_dbg!((1, 2));
    dbg!((1, 2));
    dbg!(1, 2, 3);
    // TODO my_dbg!(1, 2, 3);
}
