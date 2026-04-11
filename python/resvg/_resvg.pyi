from . import usvg as usvg
from .usvg import Tree

def render(
    tree: Tree,
    transform: tuple[float, float, float, float, float, float] | None = None,
    bg_file: str | None = None,
    bg_data: bytes | None = None,
    bg_size: tuple[int, int] | None = None,
    bg_color: tuple[int, int, int, int] | None = None,
) -> bytes: ...
def _script_entrypoint(env_args: list[str]) -> int: ...
