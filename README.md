## Usage

[環境構築](https://doc.rust-jp.rs/book/second-edition/ch01-01-installation.html)は済ませてください。  

このリポジトリをクローンします。

```
git clone https://github.com/7ma7X/reversi
cd reversi
```

ディレクトリ直下で

```
cargo build --release
```

と入力すると、`target/release/` というディレクトリの中に、`reversi` が実行形式で作成されます。外部ライブラリをいくつか使っている（後述）ので、初めてビルドする際は少し時間がかかるかもしれません。  
作成されたら

```
cd target/release/
./reversi -p 3000 -n PLAYER1
```

このようにして使えます。あるいは、ディレクトリ直下で

```
cargo run --release -- -p 3000 -n PLAYER1
```

と入力すると、ビルドと実行を一度に行うことができます。

なお、**`--release` オプションを付けないと、最適化が行われず非常に遅くなってしまうので、必ず付けるようにしましょう。**（所要時間で目に見えるレベルで遅くなります。）

## Dependencies

プログラムの作成にあたって外部ライブラリをいくつか用いているので、それについて一応注意しておきます。

### [clap](https://github.com/clap-rs/clap)  

コマンドライン引数のオプションを処理するために使用しています。

### [rand](https://github.com/rust-random/rand)

乱数ライブラリです（Rustでは乱数処理が標準ライブラリの中には存在しません）。  
OCaml で書かれた初期実装では置ける位置をランダムに選択しているので、それに倣って使っていますが、もし何かしらのアルゴリズムで決め打ちするのであれば rand は不要でしょう。

### [regex](https://github.com/rust-lang/regex)

正規表現ライブラリです（Rustでは正規表現処理が標準ライブラリの中には存在しません）。   
`commandLexer.mll` の字句解析を Rust で実装する際に、一部正規表現を用いました。