// この章では、我々は見てみましょう列挙とも呼ばれる、列挙型。
// 列挙型では、可能な値を列挙して型を定義できます。
// まず、列挙型を定義して使用し、列挙型がデータとともに意味をどのようにエンコードするかを示します。
// 次に、Option価値が何かと何もないことを表現する特に有用なenumを探そう。
// 次に、match式のパターンマッチングによって、enumの異なる値に対して異なるコードを実行する方法を簡単に見ていきます。
// 最後に、if let コード内の列挙型を処理するために使用できる便利で簡潔なイディオムの別の方法について説明します。

// 列挙型は多くの言語の機能ですが、機能は言語ごとに異なります。
// Rustの列挙型は、F＃、OCaml、Haskellなどの関数型言語の代数データ型に最もよく似ています。
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

extern "C" fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);
}
