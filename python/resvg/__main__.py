import sys

from resvg._resvg import _script_entrypoint


def entrypoint() -> None:
    raise SystemExit(_script_entrypoint(sys.argv[1:]))
