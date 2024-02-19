from resvg import render, usvg
from affine import Affine


def read_text_file(file):
    with open(file, "r") as f:
        return f.read()


def save_binary_file(file, data):
    with open(file, "wb") as f:
        f.write(bytes(data))
    print(f"binary file saved: {file}")


def test_render_default(shared_datadir):
    svg_file = shared_datadir / "a.svg"
    svg = read_text_file(svg_file)

    db = usvg.FontDatabase.default()
    db.load_system_fonts()
    options = usvg.Options.default()
    options.font_family = "Space Mono"
    tree = usvg.Tree.from_str(svg, options, db)
    tr = Affine.identity()
    data = render(tree, tr[0:6])

    png_file = shared_datadir / "a.png"
    save_binary_file(png_file, data)


def test_render_scale(shared_datadir):
    svg_file = shared_datadir / "b.svg"
    svg = read_text_file(svg_file)

    tree = usvg.Tree.from_str(
        svg, usvg.Options.default(), usvg.FontDatabase.default())
    (w, h) = tree.int_size()
    target_size = (w*20, h*20)
    tr = Affine.scale(20)
    data = render(tree, tr[0:6], bg_size=target_size)

    png_file = shared_datadir / "b.png"
    save_binary_file(png_file, data)
