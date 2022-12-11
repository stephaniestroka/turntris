# Turntris
Turntris - like Tetris, but instead of moving the stone, you rotate the board and let the stone fall to the right position.

## Introduction

Turntris is a fun project that I started to learn Rust. It compiles Rust code into WebAssembly, which can be accessed from a simple JavaScript client. The client is only responible for rendering the frontend and sending keyboard events to the backend.

## How to compile

You can follow this [Setup chapter](https://rustwasm.github.io/book/game-of-life/setup.html) to get your Rust toolchain. You'll also need npm to start the dev server.

### Compile rust to webassembly

```
wasm-pack build
```

The binary will be available pkg/.

### Start the server

```
cd www/
npm run start
```

Note: The compiled turntris code must end up under www/turntris. If that is not the case, copy it directly from pkg/.

## "I don't care about Rust - I just want to play"

Stay tuned. A proper deployment will follow shortly. 
