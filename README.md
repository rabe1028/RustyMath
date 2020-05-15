# RustyMath
 
抽象代数学のおべんきょ

## 参考

- [SwiftyMathで学ぶ数学](https://speakerdeck.com/taketo1024/swiftymathdexue-bushu-xue-chou-xiang-dai-shu-xue?slide=16)

## 設計

- 代数的構造(axiom)は，性質(property)の組と集合(set)で定義される
  - propetry = trait
  - set = type
  - axiomは複数のpropertyを合わせた性質と見れるので，traitで表す
  - TraitにGenericsを用いる理由
    - 一つの集合に対して，複数の性質がある場合の対策
      - ex) i32がAdditionとMultiplicationに対してMagmaを満たす
- Tensorの設計
  - 現在は，TensorをGenericsを用いて設計している
    - BasicArrayはTensorを表現する一つの構造体でしかない
      - CPU, GPUとかで別の構造体を用いても実装を楽にしたい
    - そのため，TensorのGenericsとして実装した
  - しかし，Genericsにしたことで，以下のデメリットが存在している
    - Trait制約がめんどくさい
      - ex)
      ```rust
      // from basic_array.rs
      impl<ElementType, Contravariant, Covariant> Tensor<ElementType, Contravariant, Covariant>
      for BasicArray<ElementType, Contravariant, Covariant>
      where
          Contravariant: HList + IndexShape + Add<Covariant>,
          Covariant: HList + IndexShape,
          Join<Contravariant, Covariant>: IndexShape,
          <Contravariant as IndexShape>::Shape: Add<
              <Covariant as IndexShape>::Shape,
              Output = <Join<Contravariant, Covariant> as IndexShape>::Shape,
          >,
      {
        ...
      }
      ```
    - 構造体の型でGenericsを受け取るのを，そのままTrait Genericsに流し込む構造になっているため，
      全てのメソッドを実装するときに，同様の制約をつける必要がある
    - BasicArray<ElementType, Covariant, Contravariant>に対して，Tensorの型は一意になるので，Genericsはいらないのでは？
    

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