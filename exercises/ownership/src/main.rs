#![allow(dead_code)]
fn main() {
    let (word_a, word_b) = words();
    let message = format!("{}{}！", word_a, word_b);
    output(message);
    //output(message);
}

// Create tuple.
fn words() -> (String, String) {
    (format!("こんにちは"), format!("世界"))
}

fn output(text: String) {
    let (text, kanji_only) = remove_hiragana(text);
    println!("{}", kanji_only);
    /*
    ゴール2：次の行をアンコメントすると何がおきるでしょうか？
    これをコンパイルを通すにはどうすれば良いでしょうか？
    */
    println!("ひらがなを抜き取ると：{:?} → {:?}", text, kanji_only);

    /*
    ゴール3：データをコピーせずにコンパイルを通すにはどおすれば良いでしょうか？
    所有権の移動のみを使って解決してください
    */
}

// return tuple
fn remove_hiragana(text: String) -> (String, String) {
    /*
     ゴール1：コンパイルを通すには何を変更すれば良いでしょうか
    */
    let mut result = String::new();
    for c in text.chars() {
        if c < 'ぁ' || 'ん' < c {
            result.push(c);
        };
    }
    // ;を打つと;の後にnil?undefinedがあるという評価となる => ;最後でreturnするやつに;書いては駄目
    // return result; と書いても良い
    (text, result)
}

// mut textとかにすると再代入可能
// 同名の別変数の定義シャドウイング fn f(mut x: String) (...) とかなら再代入も可能