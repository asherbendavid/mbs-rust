# mbs — Mandelbrot Set Viewer (Rust GUI Learning Project)

This is a hobbyist learning project to get practical, hands-on experience with GUI
programming in Rust, using a classic Mandelbrot set viewer as the vehicle. The math
is intentionally simple — the point is to learn the *mechanics* of building an
interactive desktop GUI app, one concept at a time.

## Background

Coming from VB.NET (Windows Forms) and just starting out with Kotlin (Android
Studio), this project is a deliberate step into Rust GUI development. A previous
project (a CLI text-based game) covered Rust fundamentals; this one is focused on
GUI concepts, rendering, interactivity, and eventually concurrency.

## Learning Path

The project is built in deliberate phases, each adding **one new concept** on top
of a fully working previous phase. Optimizations are intentionally deferred until
the core functionality works end-to-end — debugging one new variable at a time
beats debugging a tangle of half-finished features.

1. **Static draw** — Render the Mandelbrot set to a fixed viewing area, scaled to
   the window, recomputed fully on resize.
2. **Pan** — Allow scrolling/dragging the viewing area around the set.
3. **Zoom** — Add scroll-wheel zooming.
4. **Manual multithreading** — Split per-pixel computation across threads by hand
   (`std::thread`, slices, `Arc`) to learn safe concurrent computation in Rust.
5. **Rayon** — Replace the manual threading with the `rayon` crate's parallel
   iterators, to compare ergonomics against the hand-rolled version.
6. **Resize blur-up** — On resize, scale/fade the previous frame as a placeholder
   while the new frame recomputes (similar to map tile services scaling a cached
   image while new tiles load).
7. **Pan fill optimization** — Retain already-computed pixels when panning; only
   blank and recompute the newly revealed edge, instead of redrawing everything.

Each phase is committed once it works. Phase 4 (manual threading) is explored on
a separate branch, kept apart from `main` so the simpler rayon-based approach
(phase 5) can become the long-term `main` branch implementation going forward.

## GUI Framework

Built with [`egui`](https://github.com/emilk/egui) / `eframe`, chosen as an
easy, low-ceremony entry point into Rust GUI development (immediate-mode,
minimal boilerplate). A future revisit of this same project using a different
framework (e.g. [`iced`](https://github.com/iced-rs/iced) or
[`Slint`](https://slint.dev/)) is planned to compare architectural styles.

## Using This as Your Own Learning Path

This README is written so it can be copied directly into a prompt for an LLM
(such as Claude) to follow the same learning path. If you're starting your own
version of this project, you can paste this file's contents into a chat and ask
the assistant to guide you through each phase in order, explaining concepts as
you go rather than just generating finished code.

## Status

✅ Phase 1 complete — static draw working

🚧 Phase 2 in progress — panning
