from importlib.metadata import version as _version

from ._resvg import render, render_rgba, usvg

__version__ = _version("resvg")
__all__ = ["render", "render_rgba", "usvg"]
