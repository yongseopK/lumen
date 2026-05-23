# Lumen

Rust로 만드는 브라우저 엔진 — 브라우저가 실제로 어떻게 동작하는지 이해하기 위해 처음부터 직접 구현합니다.

> Electron도 없고, WebView도 없습니다. TCP 소켓과 직접 작성한 HTML 파서, 그리고 현실 앞의 겸손함만 있을 뿐입니다.

[English](README.en.md)

---

## 무엇을 하는가

URL을 입력받아 → 순수 TCP로 HTML을 가져오고 → DOM 트리로 파싱하고 → CSS를 적용하고 → 레이아웃을 계산하고 → 픽셀을 화면에 그립니다.

JavaScript 없음. HTTPS 아직 없음. Chrome인 척 없음.

## 아키텍처

```
URL
 └─► HTTP Client       순수 TCP, HTTP/1.1
      └─► HTML Tokenizer   문자열 → 토큰 스트림
           └─► DOM Parser      토큰 → 트리
                └─► CSS Parser     스타일시트 → 규칙 목록
                     └─► Style Resolver  DOM + CSS → 스타일 적용된 트리
                          └─► Layout Engine   → x, y, width, height 계산
                               └─► Paint Engine   → 픽셀 버퍼
                                    └─► Window Renderer  → 화면 출력
```

## 진행 현황

| # | 모듈 | 상태 |
|---|------|------|
| 1 | HTTP Client | ✅ 완료 |
| 2 | HTML Tokenizer | ⬜ |
| 3 | DOM Parser | ⬜ |
| 4 | CSS Parser | ⬜ |
| 5 | Style Resolver | ⬜ |
| 6 | Layout Engine | ⬜ |
| 7 | Paint Engine | ⬜ |
| 8 | Window Renderer | ⬜ |

## 하지 않는 것

- HTML5/CSS3 전체 스펙 지원
- JavaScript (나중에 QuickJS로 실험할 수도)
- HTTPS (아직)
- 실제 브라우저 수준의 성능

## 기술 스택

- Rust (async 없음, 프레임워크 없음, 지름길 없음)
- `std::net::TcpStream` — 네트워크
- `minifb` 또는 `pixels` — 렌더링 (미정)

## 왜 만드는가

현대 브라우저는 사실상 운영체제입니다. 이 프로젝트는 그 복잡함을 — 파서 하나씩 — 직접 이해해보기 위해 존재합니다.

## 참고자료

- [Web Browser Engineering](https://browser.engineering/) — 이 프로젝트의 출발점이 된 책
- [Servo](https://github.com/servo/servo) — Mozilla의 Rust 브라우저 엔진
