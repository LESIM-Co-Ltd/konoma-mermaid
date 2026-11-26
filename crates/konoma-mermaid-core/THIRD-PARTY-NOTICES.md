# Third-party notices

`konoma-mermaid-core` is MIT-licensed (see `LICENSE`), except where this file says otherwise.
Its license expression is `MIT AND Apache-2.0 AND ISC`. Paths below are relative to this crate.

None of the projects below is a dependency of this crate. Their code, or rules read from their
code, were copied or ported into it, so a dependency listing does not show them. The license
texts are copies of the `LICENSE` file of each project.

## dagre (Rust crate), 0.1.1 -- Apache-2.0

- Source: <https://github.com/kookyleo/dagre-rs>
- Copyright (c) kookyleo <kookyleo@gmail.com>
- Used in: `src/layout/` (a modified copy)
- License text: `src/layout/LICENSE-APACHE`
- Changes: `src/layout/PROVENANCE.md`
- Upstream ships no `NOTICE` file.

## dagre.js -- MIT

- Source: <https://github.com/dagrejs/dagre> (3.0.1-pre,
  commit `4713b59bfa05af56cf58aa01e2027adf5d2dcf88`)
- Used in: the `dagre` crate above is a port of it. `src/layout/layout/`, `src/layout/mod.rs`,
  `src/layout/util.rs`, `src/layout/cross_validate.rs` and `src/layout/cross-validate/`
  (output recorded from dagre.js, and the script that records it).

```text
Copyright (c) 2012-2014 Chris Pettitt

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

## graphlib -- MIT

- Source: <https://github.com/dagrejs/graphlib>
- Used in: `src/layout/graph/` (a Rust port, through the `dagre` crate above)

```text
Copyright (c) 2012-2014 Chris Pettitt

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

## dagre-d3-es -- MIT

- Source: <https://github.com/tbo47/dagre-es>
- Used in: `src/layout/layout/intersect.rs` (ports of its `intersect-rect`, `intersect-ellipse`
  and `intersect-polygon` functions)

```text
Original dagre-d3 copyright: Copyright (c) 2013 Chris Pettitt
Original dagre copyright: Copyright (c) 2012-2014 Chris Pettitt
Original graphlib copyright: Copyright (c) 2012-2014 Chris Pettitt

Copyright (c) 2022-2024 Thibaut Lassalle, David Newell, Alois Klink, Sidharth Vinod and dagre-es contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

## d3-shape, 3.2.0 -- ISC

- Source: <https://github.com/d3/d3-shape>
- Used in: `src/render/edges.rs` (ports of the curve generators: basis, natural, cardinal,
  Catmull-Rom, monotone, bump and step). `src/render/tests.rs` holds geometry recorded from
  running d3-shape.

```text
Copyright 2010-2022 Mike Bostock

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
```

## mermaid -- MIT

- Source: <https://github.com/mermaid-js/mermaid> (the comments in the source cite 11.12.0 and
  11.17.2; the `LICENSE` file of the two releases is identical)
- Code, tables and formulas ported or transcribed: `src/render/edges.rs` (`fixCorners`,
  `generateRoundedPath`), `src/render/shapes.rs` (shape dimensions), `src/flowchart/shapes.rs`
  (shape names and aliases), `src/chart/mod.rs` (grammar terminals), `src/chart/treemap.rs`
  (hierarchy building and number formatting), `src/render/chart/radar.rs` (radar geometry),
  `src/block.rs` (link rule), `src/render/theme.rs` (theme values)
- Grammars and database rules re-implemented in Rust: `src/architecture.rs`, `src/block.rs`,
  `src/c4.rs`, `src/gantt.rs`, `src/gitgraph.rs`, `src/journey.rs`, `src/kanban.rs`,
  `src/mindmap.rs`, `src/requirement.rs`, `src/timeline.rs`, `src/zenuml.rs`, and the files
  under `src/chart/`, `src/class/`, `src/er/`, `src/flowchart/`, `src/sequence/` and
  `src/state/`
- Drawing rules followed: files under `src/render/`

```text
The MIT License (MIT)

Copyright (c) 2014 - 2022 Knut Sveidqvist

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## d3-sankey, 0.12.3 -- BSD-3-Clause (rules reimplemented, no source text copied)

- Source: <https://github.com/d3/d3-sankey>
- Relation: the layout in `src/render/chart/sankey.rs` follows these rules of d3-sankey:
  - a node's value is the larger of what flows in and what flows out (`computeNodeValues`);
  - a node's column is the length of the longest chain of links reaching it (`computeNodeDepths`);
  - a node without outgoing links goes to the last column (`justify`);
  - columns are spaced evenly across the width (`computeNodeLayers`);
  - one scale for the whole diagram: the minimum over the columns of
    (height - (nodes - 1) * padding) / (sum of node values) (the `ky` of `initializeNodeBreadths`);
  - the bands on each side of a node are ordered by the position of their other end
    (`reorderLinks`) and stacked one after another from the node's top (`computeLinkBreadths`).
- It does not have d3-sankey's relaxation iterations or collision resolution. Nodes keep their
  declaration order inside a column, and each column is centred vertically.
- These rules are reimplemented in Rust and no source text is copied, so BSD-3-Clause is not part
  of this crate's license expression. The notice is reproduced in full.

```text
Copyright 2015, Mike Bostock
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

* Neither the name of the author nor the names of contributors may be used to
  endorse or promote products derived from this software without specific prior
  written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```
