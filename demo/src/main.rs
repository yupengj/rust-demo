mod demo1;

fn main() {
    // //猜数测试
    // demo1::SecretNumber {
    //     min: 1,
    //     max: 100,
    //     exist: 0,
    //     show_result: true,
    // }.start();

    let mut v: Vec<i32> = Vec::new();

    v.push(12);
    v.push(13);
    v.push(14);

    v.drain(..1);

    dbg!(v);
}
