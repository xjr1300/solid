# いまさらSOLID

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

## RSP: Liskov Substitution Principle: リスコフの置換原則

参考: [よき開発者になるためのS.O.L.I.D.原則](https://enjoyworks.jp/tech-blog/7729)

参考: [リスコフの置換原則（LSP）をしっかり理解する](https://qiita.com/yuki153/items/142d0d7a556cab787fad)

* **SがTの派生型であれば、プログラム内でT型のオブジェクトが使われている箇所はすべてS型のオブジェクトで置換できるべきである。**
* **派生型クラスは、スーパークラス型と置換可能でなくてはならない。**

* `リスコフの置換原則`は、言い換えると**オブジェクト指向プログラミングにおいて、サブタイプのオブジェクトはスーパータイプのオブジェクトと同じように振る舞わなければならない**という原則である。

* 関数の引数では`反変`、関数の戻り値では`共変`に従う必要がある。
  * 反変とは型の制限を弱め、 共変とは型の制限を強める。
  * 型の制限を弱めるまたは強めるとは、`型の包含関係`を示す（**実装ではない**）。
  * 例えば、スーパークラスが`A`で、派生クラスが`B`とした場合、型`A`の変数は型`B`のオブジェクトを扱えるため、`A ⊃ B`となる。
  * リスコフの置換原則と関係ないが、実装では、型`A`の実装を持った型`B`にさらに実装を追加するため、`A ⊂ B`となる。
* 関数の引数が反変である場合、引数の型がスーパータイプか、実際に引数に与える型よりも制限が弱い型を受け取る。
* 関数の戻り値が共変である場合、戻り値の型がスーパータイプか、実際に戻り値として返す型よりも制限が強い型を返す。

* また、`A`と`B`の関係は`is-a関係`であり、`B is a A`である。
* しかし、`A`を継承した`B`の実装で、`B`を`A`と同様に振る舞えない場合があり、これは`B is a A`を満たさない実装になり、リスコフの置換原則に違反する。

* リスコフの置換原則が提唱するサブタイプの振る舞いを次に示す。
  * シグネチャルール
    * 派生クラスに実装するメソッドの引数の数は、スーパータイプと同じである。
    * 派生クラスに実装するメソッドの引数の型は、スーパータイプと同じか、それより制限の少ない（反変）型を受け取る必要がある。
    * 派生クラスに実装するメソッドの戻り値は、スーパータイプと同じか、それより制限の強い（共変）型を返す必要がある。
    * 派生クラスが送出する例外は、スーパータイプと同じかその例外のサブクラスである必要がある。
  * `事前条件`と`事後条件`のルール
    * 事前条件はスーパータイプと同じか、それより弱めることができる（強めることができない）。
    * 事後条件はスーパータイプと同じか、それより強めることができる（弱めることができない）。
  * `不変条件`のルール
    * スーパータイプのプロパティが満たす不変条件を、派生クラスでも満たす必要がある。
  * `履歴（制約）`のルール
    * スーパータイプをオーバーライド、または派生クラスで追加したメソッドは、スーパータイプが許可していない方法で、プロパティを変更してはならない。
* 事前条件、事後条件、不変条件
  * 事前条件とは、処理を呼び出す前に成立すべきすべての条件を示す。
  * 事後条件とは、処理が成功した後に成立するすべての条件を示す。
  * 不変条件とは、処理前、処理中及び処理後に、データが常に満たすすべての条件を示す。

* `契約による設計`
  * 処理の成功とは、処理前に事前条件が、処理後に事後条件が、処理前、処理中及び処理後に不変条件が成立した場合である。
  * 処理は次のいずれかの場合に失敗する。
    * 処理前に事前条件が成立しない場合
    * 処理後に事後条件が成立しない場合
    * 処理前、処理中及び処理後に不変条件が成立しない場合
  * 事前条件は、メソッドの呼び出し側が、事前条件を満たすか確認して、メソッドを呼び出す必要がある。
    * また、呼び出されるクラス、または環境が、呼び出し側に事前条件を確認する方法を提供する必要がある。
  * 事後条件及び不変条件は、メソッドを呼び出される側が満たす必要がある。
  * 契約による設計においては次表に示す規則に従う必要がある。

| 当事者 | 責務 | 権利 |
| --- | --- | --- |
| 呼び出し側 | 事前条件を満たすことを確認して、処理を呼び出す | 事後条件を満たして処理が成功する |
| 呼び出される側 | 不変条件及び事後条件を満たすように、処理を適切に実装する | 事前条件がみたされて処理が適切に呼び出される |

* リスコフの置換原則は、正しい`is-a関係`を示すための原則で、それは継承すべきかどうかを判断する指針となる。
* リスコフの置換原則に照らし合わせて、`is-a関係`を満たすのであれば継承して、満たさないのであれば別のアプローチを検討する。

* 派生クラス側でスーパークラスにない検査メソッドなどを追加すると、そのメソッドを呼び出す処理が既存コードに追加される。
* これは、**既存のコードを変更せず、新しいコードを追加して拡張しなければならない**という、SOLIDの`OCP: オープンクローズドの原則`に違反する。
* このような実装が増えると、**事前に特定のメソッドを呼び出さなければならない**などの規則が出現して、そのメソッドの呼び出し忘れたコードを実装するなど不具合に通じる。

## ISP: Interface Segregation Principle: インターフェイス分離の原則

**クライアントが利用しないメソッドへの依存を、クライアントに強制してはならない。**

* 何でも処理を受け持つ`神クラス`が存在したとき、複数のクライアントがそれぞれ神クラスのひとつのメソッドを利用する場合、クライアントは神クラスのすべてのメソッドに依存することになる。
* このとき、神クラスの使用していないメソッドを変更した場合、そのメソッドを使用していないクライアントの再コンパイルと再デプロイが必要になる。
* この場合、それぞれのメソッドに対してインターフェイスを設けて、神クラスとクライアントがインターフェイスに依存するように設計する。

> そもそも、神クラスが存在すること自体が問題である。

## DIP: Dependency Inversion Principle: 依存関係逆転の法則

**上位レベルのモジュールは下位レベルのモジュールに依存してはならない。**

* ソフトウェアシステムには、そのシステムの中核となる`ドメイン知識（ビジネスルールの）`が存在する。
* ドメイン知識を`データベース（具象リポジトリ）`などに永続化する場合、ドメイン側で具象リポジトリをインポートするなど、ドメインが具象リポジトリに依存する。
* 最も重要なドメインが他に依存することは、ドメインの安定化を低下させる。
  * プレゼンテーション層 -> ユースケース層 -> ドメイン -> 具象リポジトリ
  * 下位層（上記左方法）ほど安定が低く、上位層（上記右方向）ほど安定が高い
* ドメイン層にリポジトリにインターフェースを設けて、具象リポジトリがドメインに依存するように設計する。
* 多くの場面で、ユースケース層にリポジトリのインターフェースを設けて、具象リポジトリがユースケース層に依存するように設計することがある。
  * プレゼンテーション層 -> ユースケース層（リポジトリの抽象インターフェイスを含む） <- 具象リポジトリ
  * プレゼンテーション層 -> ユースケース層 -> ドメイン（リポジトリの抽象インターフェイスを含む） <- 具象リポジトリ
* これにより、リレーショナルデータベースからキーバリューストアに永続化層を変更した場合、リポジトリの抽象インターフェースさえ守れば、キーバリューストア用の具象リポジトリを実装して既存コードに組み込むことで、修正を最小限にできる。
* 依存関係逆転の法則を適用した設計を実装する際、`DI: Dependency Injection: 依存関係の注入`が必要になる。
* DIには次の方法などがある。
  * DIコンテナによる依存関係の自動注入
  * コンストラクタインジェクション
  * メソッドインジェクション
