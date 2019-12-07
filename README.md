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