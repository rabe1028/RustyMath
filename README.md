# RustyMath
 
抽象代数学のおべんきょ

## 参考

- [SwiftyMathで学ぶ数学](https://speakerdeck.com/taketo1024/swiftymathdexue-bushu-xue-chou-xiang-dai-shu-xue?slide=16)

## 設計

- 代数的構造(axiom)は，性質(property)の組と集合(set)で定義される
  - propetry = trait
  - set = type
  - axiomは複数のpropertyを合わせた性質と見れるので，traitで表す

- trait aliasについて
  - 各代数的法則(axios)は，propertyの言い換えとするのが理想
    - ex) inverse semigroupは，quasigruop + associativity，semigroup + invertivilityの2通りの表記方法が存在してしまうから
  - [trait alias](https://github.com/rust-lang/rust/issues/41517) が安定化されたら，実装する

- 圏論について
  - 関手は，結合律と単位律を満たすが，これを満たすようなtraitを実装できない
    - Functor同士の結合（Composition）を作成しようとしたが，BinaryOperatorの定義に沿わない(BinaryOperatorはAxA->Aであるため)
    - Semigroupoidの定義では，f: A->B, g: B->Cがあった時にf . gが結合律を満たすため，BinaryOperatorを拡張する必要がある
    - しかし，FnはTraitであるため，ライフタイムが絡み，実装しにくい
    - trait genericsが実装され次第，始める