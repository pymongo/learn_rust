/*
invoice/bill: 账单，未付款商家要你付款
receipt/ticket: 付款凭证 
*/
#[test]
fn run() {
    let filename = "/home/w/files/baihai/发票/已打印未报销/receipts.csv";
    let mut total = 0.0;
    for line in std::fs::read_to_string(filename).unwrap().lines() {
        let price = line.split_once(",").unwrap().0;
        total += price.parse::<f64>().unwrap();
    }
    dbg!(total);
}