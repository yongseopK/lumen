# Lumen

A browser engine written in Rust — built from scratch to understand how browsers actually work.

> No Electron. No WebView. Just TCP sockets, a hand-rolled HTML parser, and a lot of humility.

---

## What it does

Takes a URL. Fetches the HTML over raw TCP. Parses it into a DOM tree. Applies CSS. Computes layout. Draws pixels to a window.

No JavaScript. No HTTPS (yet). No pretending this is Chrome.

## Architecture

```
URL
 └─► HTTP Client       raw TCP, HTTP/1.1
      └─► HTML Tokenizer   text → token stream
           └─► DOM Parser      tokens → tree
                └─► CSS Parser     stylesheet → rules
                     └─► Style Resolver  DOM + CSS → styled tree
                          └─► Layout Engine   → x, y, width, height
                               └─► Paint Engine   → pixel buffer
                                    └─► Window Renderer  → screen
```

## Stages

| # | Module | Status |
|---|--------|--------|
| 1 | HTTP Client | 🔄 In progress |
| 2 | HTML Tokenizer | ⬜ |
| 3 | DOM Parser | ⬜ |
| 4 | CSS Parser | ⬜ |
| 5 | Style Resolver | ⬜ |
| 6 | Layout Engine | ⬜ |
| 7 | Paint Engine | ⬜ |
| 8 | Window Renderer | ⬜ |

## Non-goals

- Full HTML5/CSS3 spec compliance
- JavaScript support (maybe later via QuickJS)
- HTTPS (yet)
- Performance anywhere near real browsers

## Built with

- Rust (no async, no framework, no shortcuts)
- `std::net::TcpStream` for networking
- `minifb` or `pixels` for rendering (TBD)

## Why

Modern browsers are essentially operating systems. This project exists to demystify them — one parser at a time.

## Reference

- [Web Browser Engineering](https://browser.engineering/) — the book that inspired this
- [Servo](https://github.com/servo/servo) — Rust browser engine from Mozilla
