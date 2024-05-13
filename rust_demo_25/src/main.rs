///
/// 合集: Rust 编程之旅
/// 博主: 深夜飚键盘 (shenyekey)
/// 平台: 抖音、B站、微信, 关注一下啦
/// 键轴: 青轴
/// 当前: rust_demo_25
///
/// 用 结构体，方法 实现 猜一猜小游戏啦
/// 回顾 猜数字小游戏的实现步骤
/// 1. 生成一个数字谜底，guess_num
/// 2. 输入一个数字，input_num
/// 3. 逻辑判断，是否猜对
/// 3.1 猜对，输出恭喜词，结束游戏
/// 3.2 猜错，输出提示词，重新输入
/// 这里在增加一种逻辑
/// 3.3 猜错，限制猜测次数，每猜一次，减少一次，限制5次
/// 5次都没猜对，结束游戏
/// 分析下，猜测次数为属性 input_count
// 安装 rand 库
// 先分析下
// 1. 游戏状态 ，运行中和结束
// 2. 游戏的属性变量，guess_num 和 input_num
// 3. 操作函数有，生成数字谜底、输入一个数字、逻辑判断
use std::io;

// 刚自动导入了
use rand::Rng;

// 1. 游戏状态 ，运行中和结束
enum GameStatus {
    RUN(String),
    OVER(String),
}

// 2. 游戏的属性变量，guess_num 和 input_num
struct Game {
    guess_num: i32,
    input_num: i32,
    // 猜测次数
    input_count: i32,
}

// 3. 操作函数有，生成数字谜底、输入一个数字、逻辑判断

impl Game {
    //
    fn new() -> Game {
        // 默认实现
        Game {
            guess_num: 0,
            input_num: 0,
            input_count: 5, // 限制 5 次
        }
    }

    // 1. 生成数字谜底
    fn create(&mut self) {
        // 数字谜底
        self.guess_num = rand::thread_rng().gen_range(1..100);
    }

    // 2. 输入一个数字
    fn input(&mut self) {
        // 用户输入
        // self.input_num
        // 键盘输入变量
        let mut num_str = String::new();
        // 键盘输入
        io::stdin().read_line(&mut num_str).expect("input error");
        // 转换数字
        self.input_num = num_str
            .trim()
            .parse::<i32>()
            .ok()
            .expect("input not number");

        // 每次输入，次数都减少一次
        self.input_count -= 1;
    }

    // 逻辑判断 , 只读属性
    // 判断完毕，返回游戏状态
    fn once(&self) -> GameStatus {
        // 猜对了，游戏结束
        if self.guess_num == self.input_num {
            GameStatus::OVER(format!(
                "猜对了！input_num = {} , guess_num = {} ",
                self.input_num, self.guess_num,
            ))
        } else {
            // 返回值是 GameStatus
            // 提示词，增加次数
            if self.input_num > self.guess_num {
                GameStatus::RUN(format!("猜大了！还剩下{}机会", self.input_count))
            } else {
                GameStatus::RUN(format!("猜小了！还剩下{}机会", self.input_count))
            }
        }
    }

    // 外部调用启动游戏
    fn run(&mut self) {
        // 1. 数字谜底
        self.create();
        // 2. 提示词,显示猜测机会
        println!("猜一猜，这个数字是多少? 共有 {} 机会",self.input_count);
        // 3. 逻辑实现
        loop {
            // 输入数字
            self.input();
            // 执行逻辑
            let status = self.once();
            // 状态判断（模式匹配）
            match status {
                // 运行中
                GameStatus::RUN(word) => {
                    println!("哎呀，{}", word);
                }
                // 猜对了
                GameStatus::OVER(word) => {
                    println!("啊哈，{}", word);
                    // 结束游戏
                    break;
                }
            }
            // 这里判断，没有次数结束游戏
            if self.input_count == 0 {
                // 提示词
                println!("机会用完啦");
                break;
            }
        }
        println!("游戏结束啦");
    }
}

fn main() {
    // 实例化游戏
    let mut game = Game::new();
    game.run();
}
