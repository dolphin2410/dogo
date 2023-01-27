# dogo
Maven 저장소에서 최신 아티팩트 불러오기


## 설치
1. 직접 컴파일하기 (Rust 필요함)
```bash
cargo install --path .
```

2. 릴리즈에서 다운받기

## 사용법
1. Github
```bash
dogo github @<owner>/<repo>
```

예시)
```bash
dogo github @monun/tap
dogo github @papermc/paper
```

2. 직접추가 (닉네임 설정)
```bash
dogo set <nickname> <notation>
dogo search <nickname>

# (기본값 = mavenCentral) 필수 x
dogo repo <nickname> <repository>
```


```bash
dogo set tap io.github.monun:tap-api
dogo search tap

dogo set paper io.papermc.paper:paper-api
dogo repo paper https://repo.papermc.io/repository/maven-public
dogo search paper
```

결과는 아래 비슷하게 나온다
![](./example.png)