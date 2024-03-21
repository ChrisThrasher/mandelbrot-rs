# mandelbrot-rs

SFML-based Mandelbrot viewer program.

Reimplementation of https://github.com/ChrisThrasher/mandelbrot.

<p float="middle">
    <img src="docs/mandelbrot.png" width="300"/>
    <img src="docs/zoomed.png"     width="300"/>
</p>

# Building & Running

```
cargo run --release
```

# Controls

| Action            | Control         |
| ----------------- | --------------- |
| Go to point       | Click           |
| Zoom              | Scroll (or W/S) |
| Pan               | Arrow keys      |
| Change iterations | [ and ]         |
| Reset view        | R               |
