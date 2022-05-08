🚧 開発中につき少々お待ち下さい (2022-05 公開予定) 👩‍💻

## repos 構造

- (repos-root)
  - .build/                    ; pub -> target/lib   , rust build system for final lib
    * build.rh                 ; preprocessor
    * lib.rs                   ; final lib source
    * (resource.includable.rs) ; (auto generated lib resource dictionary source code)
  - pub.debug/                 ; src -> pub.debug/   , web build system for debug
  - pub.release/               ; src -> pub.release/ , web build system for release
  - src/                       ; ui sources
    - UI1/
    - UI2/
    - ...

uis は UI (=web UI部品) 開発がメインなので基本的にこのreposで作業するデザイナーさんの扱いを中心に構造を設計しています。
最後は rust の lib として unaf-cli/unaf-svr から dependencies で回収されますが、その部分は unaf-uis 自体の主体ではないため、
rust 関連のビルドソースなどは .build にひっそり配置し、 src は UI (=web UI部品) 開発向けに振り切っています。

そういうわけで、この repos の開発はほぼ" package.json と yarn で src から pub.release/pub.debug へ web UI 部品を開発する"
ところになっています。
