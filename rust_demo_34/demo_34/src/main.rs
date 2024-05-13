
use demo_34_libs::add;

// 第三方包
use rand::Rng;

fn main() {
    println!("Hello, world! add {}",add(1,1));

    println!("随机数字 {}",rand::thread_rng().gen_range(1..100));
}
