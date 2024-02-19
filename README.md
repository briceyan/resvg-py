# resvg-py

![title](https://github.com/briceyan/resvg-py/blob/main/images/a.png?raw=true)

Resvg-py is a Python package that provides high-performance SVG rendering by wrapping the `resvg` Rust library using PyO3. This package allows Python applications to easily render SVG files to various image formats with high fidelity and performance.

See https://github.com/RazrFalcon/resvg for more info about resvg.

This project is inspired by https://github.com/yisibl/resvg-js. The image displayed above has been modified from a test image(SVG) included in this project and was converted using this package.

## Installation

```sh
pip install resvg affine
```

## Usage

Here's a simple example of how to use resvg-py to convert an SVG file to a PNG image:

```python
from resvg import render, usvg
import affine

with open("a.svg", "r") as f:
    svg = f.read()

db = usvg.FontDatabase.default()
db.load_system_fonts()

options = usvg.Options.default()
options.font_family = "Space Mono"

tree = usvg.Tree.from_str(svg, options, db)
tr = Affine.identity()
data = render(tree, tr[0:6])

with open("a.png", "wb") as out:
    out.write(bytes(data))
```

File content of `a.svg`

```xml
<svg width="600" height="300" xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink">
  <defs>
    <linearGradient x1="0%" y1="45%" x2="100%" y2="55%" id="b">
      <stop stop-color="#FF8253" offset="0%" />
      <stop stop-color="#DA1BC6" offset="100%" />
    </linearGradient>
    <path id="a" d="M0 0h600v300H0z" />
  </defs>
  <g fill="none" fill-rule="evenodd">
    <use fill="url(#b)" xlink:href="#a" />
    <text x="50%" y="20%" font-size="48" fill="#FFF" dominant-baseline="middle" text-anchor="middle">
      resvg-py</text>
    <text x="50%" y="40%" font-size="24" fill="#FFF" dominant-baseline="middle" text-anchor="middle">
      a Python binding of resvg</text>
    <text x="50%" y="60%" font-size="48" fill="#FFF" dominant-baseline="middle" text-anchor="middle">
      resvg</text>
    <text x="50%" y="80%" font-size="24" fill="#FFF" dominant-baseline="middle" text-anchor="middle">
      an SVG rendering library in Rust
    </text>
  </g>
</svg>
```

## Affine Transformation

The resvg-py package utilizes the [affine](https://github.com/rasterio/affine) library for the convenient creation of affine transformation matrices, thereby eliminating the need to directly wrap the [Transformation](https://docs.rs/tiny-skia/0.11.4/tiny_skia/struct.Transform.html) class from tiny_skia.

It's important to note that when providing a 6-element tuple as the second parameter to the `render()` function, this tuple should be in row-major format. This detail is crucial for those who might calculate the transformation matrix using other methods or libraries before passing it to resvg-py.
