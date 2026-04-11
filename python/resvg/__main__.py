import sys

from ._resvg import _script_entrypoint  # pyright: ignore[reportPrivateUsage]


def entrypoint() -> None:
    raise SystemExit(_script_entrypoint(sys.argv[1:]))
