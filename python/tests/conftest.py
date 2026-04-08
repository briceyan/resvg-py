import shutil
from pathlib import Path

import pytest

TESTS_ROOT = Path(__file__).resolve().parent
DATA_ROOT = TESTS_ROOT / 'data'


@pytest.fixture
def shared_datadir(request: pytest.FixtureRequest, tmp_path: Path) -> Path:
    tmp_data_path = tmp_path / 'data'
    shutil.copytree(DATA_ROOT, tmp_data_path)
    return tmp_data_path
