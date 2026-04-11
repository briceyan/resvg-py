from pathlib import Path

from resvg import render, usvg
from affine import Affine


def save_binary_file(file: Path, data: bytes):
    file.write_bytes(data)
    print(f"binary file saved: {file}")


def test_render_default(shared_datadir: Path) -> None:
    svg_file = shared_datadir / "a.svg"
    svg = svg_file.read_text(encoding='utf-8')

    options = usvg.Options.default()
    options.font_family = "Space Mono"
    tree = usvg.Tree.from_str(svg, options)
    tr = Affine.identity()
    data = render(tree, tr[0:6])
    assert isinstance(data, bytes)

    png_file = shared_datadir / "a.png"
    save_binary_file(png_file, data)


def test_render_scale(shared_datadir: Path) -> None:
    svg_file = shared_datadir / "b.svg"
    svg = svg_file.read_text(encoding='utf-8')

    tree = usvg.Tree.from_str(svg, usvg.Options.default())
    (w, h) = tree.int_size()
    target_size = (w*20, h*20)
    tr = Affine.scale(20)
    data = render(tree, tr[0:6], bg_size=target_size)
    assert isinstance(data, bytes)

    png_file = shared_datadir / "b.png"
    save_binary_file(png_file, data)
