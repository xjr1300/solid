# SOLID

## SRP: Single Responsibility Principle: 単一責任の原則

**モジュールを変更する理由は1つに限る。**

* `単一責任の原則`とは、**モジュール、クラスまたは関数は、単一の機能について責任を持ち、その機能をカプセル化するべきである**という原則である。
* ソフトウェアシステムを開発または改修する目的は、**ステークホルダー**を満足させるためである。
* 単一責任の原則を、**同じような要望を持ったユーザーやステークホルダーのグループに対して責任を負うべきである**と言い換えることができる。
* ここで、ここで言及している`モジュール`は、ソースファイル、またはいくつかの関数やデータをまとめた`凝集性`のあるものである。
* そして、そのモジュールは、たったひとつの`グループ`に対して責任を負うようにする。
* `凝集性のある`という言葉は、単一責任の原則（SRP）を匂わせており、そのグループに対する責任を負うコードをモジュールにまとめることを強制する。
* なお、最下位レベルの`モジュール`にある巨大な関数をリファクタリングして、小さな関数に分割するが、**これを単一責任の原則とは呼ばない**。

> まず、`DDD`における`コンテキスト`境界でモジュールを分割する。
> `コンテキスト`で分割した後は、必要に応じて**ひとつの課題を解決することを試みた時に同時に修正が発生するもの**でモジュールをまとめる。
> 分割だけでなく、`まとめる`という考え方も重要ではないか。

## OCP: Open Closed Principle: オープン・クローズドの原則

**ソフトウェアの構成要素（モジュール、クラス、関数など）は拡張に対しては開いていて、修正に対しては閉じているべきである。**

* `オープン・クローズドの原則`とは、拡張（機能追加など）にたいして、既存のコードを変更せずに、新しいコードを追加することで対応できるようにすることである。

> 抽象基本クラスとと具象派生クラスに分けて実装して、抽象基本クラスから派生したクラスを定義しして、その派生具象クラスでベースとなった抽象基本クラスの純粋仮装関数をオーバーライドして、機能を拡張することを想像すると理解しやすい。
> この場合、既存のコードを変更する必要がないか、変更は最小限に抑えられると考えられる。
