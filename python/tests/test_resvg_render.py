import pytest
from pathlib import Path

from resvg import render, usvg
from affine import Affine

PNG_MAGIC = b"\x89PNG\r\n\x1a\n"


def save_binary_file(file: Path, data: bytes):
    file.write_bytes(data)
    print(f"binary file saved: {file}")


def test_render_default(shared_datadir: Path) -> None:
    svg_file = shared_datadir / "a.svg"
    svg = svg_file.read_text(encoding='utf-8')

    options = usvg.Options.default()
    options.load_system_fonts()
    options.font_family = "Space Mono"
    tree = usvg.Tree.from_str(svg, options)
    tr = Affine.identity()
    data = render(tree, tr[0:6])
    assert data[:8] == PNG_MAGIC

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
    assert data[:8] == PNG_MAGIC

    png_file = shared_datadir / "b.png"
    save_binary_file(png_file, data)


def test_render_bg_color(shared_datadir: Path) -> None:
    svg = (shared_datadir / "b.svg").read_text(encoding='utf-8')
    tree = usvg.Tree.from_str(svg, usvg.Options.default())
    data = render(tree, (1, 0, 0, 1, 0, 0), bg_color=(255, 255, 255, 255))
    assert data[:8] == PNG_MAGIC


def test_render_bg_data(shared_datadir: Path) -> None:
    svg = (shared_datadir / "b.svg").read_text(encoding='utf-8')
    tree = usvg.Tree.from_str(svg, usvg.Options.default())
    bg = render(tree, (1, 0, 0, 1, 0, 0))
    data = render(tree, (1, 0, 0, 1, 0, 0), bg_data=bg)
    assert data[:8] == PNG_MAGIC


def test_render_bg_file(shared_datadir: Path) -> None:
    svg = (shared_datadir / "b.svg").read_text(encoding='utf-8')
    tree = usvg.Tree.from_str(svg, usvg.Options.default())
    bg_path = shared_datadir / "bg.png"
    bg_path.write_bytes(render(tree, (1, 0, 0, 1, 0, 0)))
    data = render(tree, (1, 0, 0, 1, 0, 0), bg_file=str(bg_path))
    assert data[:8] == PNG_MAGIC


def test_render_exclusive_bg_args(shared_datadir: Path) -> None:
    svg = (shared_datadir / "b.svg").read_text(encoding='utf-8')
    tree = usvg.Tree.from_str(svg, usvg.Options.default())
    bg = render(tree, (1, 0, 0, 1, 0, 0))

    with pytest.raises(ValueError):
        render(tree, (1, 0, 0, 1, 0, 0), bg_data=bg, bg_size=(10, 10))
