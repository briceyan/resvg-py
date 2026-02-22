from importlib.metadata import version as _version

from ._resvg import render, usvg

__version__ = _version("resvg")
__all__ = ["render", "usvg"]
