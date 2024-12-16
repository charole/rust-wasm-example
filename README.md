## Intro
### 웹어셈블리란 무엇인가?
웹어셈블리(WebAssembly, 줄여서 Wasm)는 웹에서 실행되는 바이너리 형식의 코드와 해당 코드의 텍스트 표현입니다. 복잡한 연산이나 CPU 집약적인 작업을 더 빠르게 처리할 수 있도록 설계되었으며, 게임, 비디오 편집기, 이미지 처리와 같은 고성능 애플리케이션에서 널리 사용됩니다.

### 웹어셈블리 특징
1. `고성능`: 웹어셈블리는 네이티브 코드에 가까운 속도를 제공합니다. 이는 C, C++, Rust 등의 언어로 작성된 프로그램을 웹 환경에서 실행할 수 있게 합니다.
2. `이식성`: 운영체제나 브라우저에 상관없이 실행됩니다. 주요 브라우저(크롬, 사파리 등)에서 지원되며, 플랫폼 간 호환성이 뛰어납니다. (모든 브라우저에서 지원하는 것은 아닙니다.)
3. `보안성`: 샌드박스 환경(격리된 환경)에서 실행되므로 보안이 강화된 구조를 가지고 있습니다.
4. `JavaScript와의 상호운용성`: JavaScript와 함께 사용되도록 설계되었습니다. JavaScript 코드에서 웹어셈블리 모듈을 불러와 사용할 수 있습니다.

### 웹어셈블리 작동 원리
1. 소스 코드 작성: `C`, `C++`, `Rust` 등의 언어로 소스 코드를 작성
2. 컴파일: 소스 코드를 웹어셈블리(Wasm) 바이너리로 컴파일합니다. 이 과정에서 `emscripten`, `wasm-bindgen`과 같은 도구를 사용할 수 있음
3. 브라우저에서 실행: 컴파일된 웹어셈블리 모듈은 브라우저의 JavaScript 엔진에서 실행됨 (웹어셈블리는 별도의 플러그인 없이 브라우저에서 네이티브 성능에 가까운 실행 가능)

### 웹어셈블리와 자바스크립트의 차이점
1. 성능
   - `웹어셈블리`는 컴파일된 바이너리 형식으로 제공되므로 브라우저가 이를 바로 실행할 수 있습니다.
   - `JavaScript`는 인터프리터 방식으로 실행되거나 Just-In-Time(JIT) 컴파일이 필요합니다.
2. 언어 선택
   - `JavaScript`는 `JavaScript` 언어로만 작성됩니다.
   - `웹어셈블리`는 `C`, `C++`, `Rust` 등 다양한 언어를 지원합니다.
3. 사용 목적
   - `JavaScript`는 DOM 조작, 이벤트 핸들링 등 웹 개발에 최적화된 언어입니다. 
   - `웹어셈블리`는 고성능이 필요한 연산, 복잡한 알고리즘, 멀티미디어 처리 등에 사용됩니다.
4. 상호 운용성 
   - `웹어셈블리` 모듈은 `JavaScript`에서 호출할 수 있으며, JavaScript도 웹어셈블리에서 호출 가능합니다. 
   - `JavaScript`는 높은 수준의 프로그래밍 작업(예: UI 작업)에 적합하지만, 웹어셈블리는 낮은 수준의 작업(예: 수학 연산)에 적합합니다.

### 웹어셈블리의 주요 사용 사례
- `게임 개발`: `Unity`, `Unreal Engine`으로 제작된 고성능 게임의 웹 버전을 웹어셈블리로 빌드
- `비디오 및 이미지 처리`: FFMPEG을 웹어셈블리로 컴파일하여 브라우저에서 동영상 편집
- `과학 계산 및 머신러닝`: 복잡한 계산을 빠르게 처리하는 라이브러리 제공
- `기존 소프트웨어 이식`: 데스크톱 애플리케이션을 웹 환경으로 쉽게 이식

### 웹어셈블리의 한계
- `초기 학습 곡선`: 기존 JavaScript 개발자들에게는 생소한 환경일 수 있습니다.
- `디버깅 어려움`: 바이너리 형식이므로 디버깅이 어렵고, 소스맵이나 추가적인 도구가 필요합니다.
- `DOM 접근 제약`: DOM에 직접 접근할 수 없으며, JavaScript를 통해 접근해야 합니다.
- `파일 크기`: 웹어셈블리 모듈 크기가 JavaScript보다 클 수 있어 초기 로딩 시간이 증가할 수 있습니다.

## Install
### rust 설치
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### cargo 설치
`cargo`는 Rust의 패키지 매니저입니다.(`npm`, `yarn`, `pnpm`과 동일) (`rust` 설치 시 자동으로 설치됩니다.)
```bash 
curl https://sh.rustup.rs -sSf | sh -s -- -y
```

### cargo-generate 설치
`cargo-generate`는 Rust 프로젝트 템플릿을 생성하는 도구입니다.
```bash
cargo install cargo-generate
```

### wasm-bindgen-cli, wasm-pack 설치
- `wasm-bindgen-cli`는 WebAssembly와 JavaScript 간의 통신을 도와주는 도구입니다. (javascript webassmbly 간의 데이터 타입도 자동으로 맞춰줍니다.)
- `wasm-pack`은 WebAssembly 프로젝트를 빌드하고 테스트하고 배포하는 도구입니다.
```bash
cargo install wasm-bindgen-cli
cargo install wasm-pack
```


## rustc, rustup, cargo 차이점
- `rustc`는 Rust 컴파일러입니다. (`tsc` 유사)
- `rustup`은 Rust 버전 관리 도구입니다. (`nvm` 유사) 
- `cargo`는 Rust 패키지 매니저입니다. (`pnpm` 유사)
  - `Cargo.toml` : Rust 프로젝트의 메타데이터와 의존성을 정의하는 파일입니다.

## 웹과 어셈블리 연동
wasm-app template를 적용합니다.
```bash
npm init wasm-app web
```
### pnpm install
web 폴더로 이동 후 package들을 설치합니다.
```bash
cd web
pnpm install
```

### webpack.config.json 수정
webpack5 설치 후 webpack.config.json을 수정합니다.
```js
experiments: {
    asyncWebAssembly: true
}
```

### web(웹)에서 wasm 사용하기
web/package.json을 수정합니다.
```json
{
  "dependencies": {
    "my-wasm-project": "../pkg"
  }
}
```

### wasm build
프로젝트 루트 경로에서 wasm-pack을 이용해 rust 코드를 웹어셈블리로 컴파일 합니다.
```bash
wasm-pack build
```

### pnpm start
web에서 pnpm start를 실행합니다.
```bash
pnpm start
```


## 레퍼런스
([WebAssembly가 뭐야? 너가 JavaScript를 대체한다고!?!?](https://hooninedev.com/240522/))\
([웹 어셈블리에 대한 오해와 사실들](https://velog.io/@kimgh06/%EC%9B%B9-%EC%96%B4%EC%85%88%EB%B8%94%EB%A6%AC%EC%97%90-%EB%8C%80%ED%95%9C-%EC%98%A4%ED%95%B4%EC%99%80-%EC%82%AC%EC%8B%A4%EB%93%A4))\
([rust tutorial](https://rustwasm.github.io/book/game-of-life/hello-world.html))
([rust handbook](https://rinthel.github.io/rust-lang-book-ko/foreword.html))