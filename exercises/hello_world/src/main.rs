/*
ゴール：
1. コンパイルし、実行してください
2. worldを自分の名前に置き換えてみてください
3. 変数 name を使って2を実現してください
4. println!("{}", name)を試しください
5. println!("{}", name)とprintln!("{:?}", name)の違いを確認してください
6. greetingsを呼ぶように変更してください
*/
#![allow(dead_code)]

fn main() {
    let name = format!("hello bokuweb");
    // println!("{:?}", name);
    greetings(name.clone());
    greetings(name.clone());
    greetings(name);

    let value = 1;
    // 数値は自動的に内部でcloneが走る
    println!("{} + 1 = {}", value, add_one(value));
}

fn add_one(a: i32) -> i32 {
    a + 1
}

fn greetings(name: String) {
    println!("Hello, {}!", name);
}
