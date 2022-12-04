mod demo1;

fn main() {
    //猜数测试
    demo1::SecretNumber {
        min: 1,
        max: 100,
        exist: 0,
        show_result: true,
    }.start();
}