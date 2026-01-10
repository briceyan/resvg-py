from importlib.metadata import PackageNotFoundError, version

from ._resvg import __doc__ as __doc__
from ._resvg import render, usvg

try:
    __version__ = version("resvg")
except PackageNotFoundError:
    __version__ = "unknown"

__all__ = ["render", "usvg"]
