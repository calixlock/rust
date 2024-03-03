### ◆ RUST cargo 관련 명령어

- #### 자동 정렬 기능 먹통시 적용

```sh
cargo fmt
```

- #### 컴파일러의 권고를 바타으로 수정

```sh
cargo fix
```

- #### Clippy 린트(코드 분석 린트 집합) 코트 품질 향상

```sh
cargo clippy
```

- #### 실시간으로 결과를 계속 확인 가능함

```sh
# install
cargo watch crate설치 후 가능

# execute
cargo watch -q -c -x 'run -q'
```
