import pytest
from pathlib import Path

from resvg import usvg


def test_list_fonts_initially_empty() -> None:
    opts = usvg.Options.default()
    assert opts.list_fonts() == []


def test_load_system_fonts() -> None:
    opts = usvg.Options.default()
    opts.load_system_fonts()
    assert isinstance(opts.list_fonts(), list)


def test_load_font_file(shared_datadir: Path) -> None:
    opts = usvg.Options.default()
    opts.load_font_file(str(shared_datadir / "SpaceMono-Regular.ttf"))
    assert "Space Mono" in opts.list_fonts()


def test_load_fonts_dir(shared_datadir: Path, tmp_path: Path) -> None:
    subdir = tmp_path / "fonts" / "subfont"
    subdir.mkdir(parents=True)
    for name in ("SpaceMono-Regular.ttf", "Roboto-Regular.ttf"):
        (subdir / name).write_bytes((shared_datadir / name).read_bytes())
    opts = usvg.Options.default()
    opts.load_fonts_dir(str(tmp_path))
    fonts = opts.list_fonts()
    assert "Space Mono" in fonts
    assert "Roboto" in fonts


def test_load_font_data(shared_datadir: Path) -> None:
    opts = usvg.Options.default()
    opts.load_font_data((shared_datadir / "SpaceMono-Regular.ttf").read_bytes())
    assert "Space Mono" in opts.list_fonts()


def test_load_font_file_not_found() -> None:
    opts = usvg.Options.default()
    with pytest.raises(OSError):
        opts.load_font_file("/nonexistent/path/font.ttf")


def test_load_fonts_dir_not_found() -> None:
    opts = usvg.Options.default()
    with pytest.raises(FileNotFoundError):
        opts.load_fonts_dir("/nonexistent/dir")


def test_load_font_data_invalid_is_ignored() -> None:
    opts = usvg.Options.default()
    opts.load_font_data(b"not a font")
    assert opts.list_fonts() == []
