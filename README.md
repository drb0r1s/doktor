# DOKTOR

**A web rendering language with its own compiler, layout engine, runtime control, renderer, scripts and development server. The language is built from scratch, from tokenizing to pixel-drawing.**

> **Latest Status: Actively In Development.** APIs, syntax, and internals are all subject to change. Not ready for production use, but the core pipeline is real, working, and gets more capable every week.

---

## What is DOKTOR?

DOKTOR is a web rendering language for building User Interfaces. It is positioned somewhere between HTML/CSS and a native layout engine. It contains a full custom toolchain that takes DOKTOR source code all the way to pixels on a screen, with no browser layout engine involved.

```
[Group: main | <height: 100%, width: 100%, alignment: center>] {
    [Text | content: DOKTOR LANGUAGE | <content_size: 60, content_color: #e5e510>]

    [Group: author | <alignment: center>] {
        [Text | content: Author: | <content_size: 30, margin_right: 16>]
        [Text | content: Boris Marinkovic | <content_size: 30, content_color: #e5e510>]
    }
}
```

That's real DOKTOR syntax, compiled and rendered by the pipeline in this repository.

## Why build a whole language + renderer from scratch?

Because it sits at the intersection of a few genuinely hard, still-open problems, and that combination is rare to get to work on in one project.

Tokenizing and parsing belongs to compiler-theory, but resolving a semantic tree into a layout-ready representation with wrapping, alignment, margins, padding, percentage sizing, and scroll-aware clipping all interacting correctly is where it turns into real layout-engine design, which is the same category of problem browser engines and native UI toolkits spend years on. And once layout is solved, none of it means anything until it's turned into GPU work: packing a tree into a binary format, designing WebGL shaders for borders and fills, coordinating a Canvas 2D text layer on top, that's systems and graphics programming, not web development.

There's also a real performance thesis underneath the curiosity: HTML/CSS's layout and rendering model is general-purpose by necessity, it has to support arbitrary documents, arbitrary styling, decades of backward compatibility. A purpose-built pipeline with no DOM, no CSS cascade, no browser flexbox/grid solver, and a compact binary hand-off between layout and GPU has room to be meaningfully faster for the specific class of applications it targets: structured UIs and documents, not arbitrary web pages.

## The pipeline

DOKTOR source doesn't go straight to the screen, it flows through a real multi-stage compiler and runtime:

```
.doktor source
      │
      ▼
Tokenizer -> Parser -> Resolver (doktorc: frontend)
      │
      ▼
Shaper -> Scroller -> Painter (doktorc: middle)
      │
      ▼
Packer (doktorc: backend)
      │
      ▼
.doktorb binary packet buffer (doktorr)
      │
      ▼
WASM runtime -> WebGL + Canvas 2D renderers (doktorr + doktorw)
```

- **Tokenizer / Parser / Resolver**: turn raw `.doktor` text into a validated, typed semantic tree, with real error and warning reporting (unrecognized block types, invalid style values, illegal children, etc.)
- **Shaper**: a real layout engine: bottom-up sizing, top-down positioning, line-wrapping, alignment (start/center/end on both axes), margins, padding, percentage dimensions, free-form absolute positioning, and locked dimensions.
- **Scroller**: post-layout scroll offset application, clipping, and scrollable-region tracking, so scrolling doesn't require re-running layout from scratch.
- **Painter / Packer**: flattens the final tree into a fixed-stride binary packet format shared between Rust and JavaScript.
- **Runtime (WASM)**: compiled to WebAssembly, driving hit-testing, scroll interaction, and incremental repaints from the browser.
- **Renderers**: hand-written WebGL shaders for rectangles, borders (solid/dashed/dotted), and images, plus a Canvas 2D text layer.

## What already works

- Full compiler pipeline: tokenizer -> parser -> resolver -> shaper -> scroller -> painter -> packer
- Real layout: wrapping, alignment, margin/padding, percentage sizing, locked dimensions, free-positioning
- Overflow handling: clipped, visible, and scrollable content
- Scrolling: wheel input, clamped bounds, scroll-position-aware scrollbars, correct z-ordering
- A tag-based styling system (`Styles` / `Style` blocks), it enables defining styles once, apply them anywhere by tag
- Click and hit-testing straight through to the semantic tree
- Cross-platform CLI tooling (`doktor compile`, `doktor update`) for the development loop

## What's next

- A full scripting system for interactivity
- A live DOKTOR server for real projects (not just single-file demos)
- More block types, attributes, and style properties

## The repository

| Repository | What it is |
|---|---|
| `doktorc` | The compiler: tokenizer, parser, resolver, shaper, scroller, painter, packer |
| `doktorr` | The Rust/WASM runtime that drives the browser-side pipeline |
| `doktorw` | The JavaScript runtime: WebGL + Canvas 2D renderers, event handling |
| `doktorss` | Cross-platform CLI for the dev loop (`doktor compile`, `doktor update`) |

## Future

This is a solo project built out of genuine curiosity about compilers, layout engines, and systems-level graphics programming, it is not a framework wrapper.
