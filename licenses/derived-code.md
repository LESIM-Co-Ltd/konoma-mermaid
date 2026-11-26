## Code derived from other projects in the konoma crates -- MIT

`konoma-widgets` (`src/markdown/render.rs`) contains row-building methods derived from the
`TextWriter` of `tui-markdown`, which is not a dependency. Source:
<https://github.com/joshka/tui-markdown>. License text (`LICENSE-MIT` at the commit of the 0.3.8
release):

```text
MIT License

Copyright (c) 2024 Josh McKinney

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

`konoma-widgets` (`src/kitty.rs`) also ports code of `ratatui-image`. That crate is linked into the
program as well, so its license is in the crate list above.
