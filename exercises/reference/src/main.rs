/*
ゴール：
1. データをコピーしなくても、greetを呼び出せるようにしてください
2. "Hello dear rustaceans"ではなく、"Hello rustaceans"となるように、
   2回目のgreetの呼び出しを行なってください
3. 2をスライスを使って実現してください
*/

fn main() {
    let name = format!("dear rustaceans");
    // &mut を使うと参照先の変更も可能
    let r = &name[..];
    let s = &name[5..]; // nameの5から末尾
                        // 範囲は5文字目じゃななくて5byte目
                        // slice
    greet(r);
    greet(s);
}

fn greet(name: &str) {
    // 内部で自動的に参照から実態に変えられる => *name　としなくてよい 
    println!("Hello {}", name);
}

// &Stringからマジカルに&strになるので引数で受け取るときは&Stringより&strを使う方が便利です
/*
format!("こんにちは")
と
"こんにちは".to_string()
は前者はformat!としての機能を活かしてないから同じものと考えてしまっていいのかなあ？
 => 同じ
 */