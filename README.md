# konoma-mermaid

Mermaid diagrams drawn without a browser or Node: the renderer of
[konoma](https://github.com/LESIM-Co-Ltd/konoma), a terminal file browser, and its plugin.

![A mermaid flowchart with decisions, a database node and a "Release" subgraph, drawn as a picture in the terminal.](https://raw.githubusercontent.com/LESIM-Co-Ltd/konoma-mermaid/main/assets/flowchart.png)

![A mermaid sequence diagram of a login request across four participants, with an alt/else block and a note.](https://raw.githubusercontent.com/LESIM-Co-Ltd/konoma-mermaid/main/assets/sequence.png)

| Crate | What it is |
| --- | --- |
| [`konoma-mermaid-core`](crates/konoma-mermaid-core/README.md) | Library: Mermaid source to SVG, with its own parsers and layout. |
| [`konoma-mermaid`](crates/konoma-mermaid/README.md) | konoma's `mermaid` plugin, and a program that shows one diagram file. |

## Try it

From a clone of this repository:

```sh
printf 'flowchart LR\n  A[Source] --> B[SVG]\n' > flow.mmd
cargo run -p konoma-mermaid -- flow.mmd
```

Press `q` to quit. If you have konoma, it can show the gallery of diagram kinds in `samples/`
(the standalone program does not open Markdown):

```sh
konoma samples/mermaid.md
```

## Build and test

Rust 1.92 or later.

```sh
cargo build
cargo test
```

CI runs on macOS and Ubuntu, with and without default features, and also checks formatting and
clippy. Some tests measure labels against installed fonts. On Ubuntu CI installs `pkg-config`,
`cmake`, `libssl-dev`, `zlib1g-dev`, `fontconfig-config`, `fonts-dejavu-core`,
`fonts-freefont-ttf`, `fonts-droid-fallback` and `fonts-noto-color-emoji`.

## License

MIT (see [LICENSE](LICENSE)), except `crates/konoma-mermaid-core/src/layout/`, a modified copy of
the `dagre` crate by kookyleo that stays under Apache-2.0. See its
[license](crates/konoma-mermaid-core/src/layout/LICENSE-APACHE) and
[provenance](crates/konoma-mermaid-core/src/layout/PROVENANCE.md).
