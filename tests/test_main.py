from pathlib import Path
from unittest.mock import patch

from genpnize import main


def test_genpnize_from_file_path(capsys):
    resource_dir_path = Path(__file__).parent / "resources"
    input_path = resource_dir_path / "file" / "input.txt"
    with patch("sys.argv", ["genpnize", str(input_path)]):
        main()

    expected = (resource_dir_path / "file" / "expected.txt").read_text()
    assert capsys.readouterr().out == expected
