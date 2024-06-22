# Mini-Pol Lang - Overview
Pol Langのシンタックスを持つ、必要最低限の機能を備えた言語

## Purpose
- Pol Langを作るにあたって直面した問題を小さい言語を実際に実装して使ってみることで解決法を導き出すため。

## Language Features
### Summary
- 式指向言語
- 必要最低限のシンタックス
- 必要最低限の型
- インタプリタ

### Syntax
#### Comment
- commentは`//`で囲みます。例えば、`//これはコメントです。//`と書きます。

#### Entory Point
- エントリーポイントは`main`です。
- `main`関数には引数がなく、返り値もありません。
- エントリーポイントは実質定数であり、プログラム全体を通して変更されません。

#### Bind
- `:=`はバインド演算子で返り値を省略できます。
- 変数は不変です。
- 返り値は一つしか返せません。
- 副作用のあるバインドの返り値はありません。

**Variable**
- `var_name := "Hello World"`でバインドできます。

**Function**
- `fn_name |parameters| : return_type = expression`で関数をバインドできます。

#### Call
**Variable**
- `var_name`で変数を呼び出せます。

**Function**
- `fn_name arguments`で関数を呼び出せます。

#### Conditional branching
- `if condition expression condition expression el expression`で条件分岐できます。

#### Operator
- 演算（比較、数値、文字列演算）は全てポーランド記法です。

**Arithmetic operations**
- int型の演算には`+`, `-`, `*`, `/`, `%`が使えます。
- `+`はstr型にも使えます。

**Comparison operations**
- 全ての型に対して`=`が使えます。
- int型には`>`と`<`が使えます。
- bol型には`!`, `&`, `|`が使えます。

### Types
**int**
- 例: `3`, `-55`

**str**
- 例: `"hello"`

**bol**
- 例: `true`, `false`

### Default Library
- `in` : str
- `out` |str|
- `outln` |str|
- `str` |int or bol|

## Language design principles
- blockは存在せず、全てのバインドの中身は引数か標準入出力だけによって変化します。
- 型の数を最小限に抑え、学習コストを引き下げます。

## Development environment
- cargo
