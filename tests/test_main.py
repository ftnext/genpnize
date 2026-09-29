import subprocess
import sys
import sysconfig
from pathlib import Path
from unittest.mock import patch

from genpnize import main


def test_genpnize_from_text(capfd):
    resource_dir_path = Path(__file__).parent / "resources"
    input_path = resource_dir_path / "file" / "input.txt"
    with patch("sys.argv", ["genpnize", input_path.read_text(encoding="utf-8").replace("\n", "")]):
        main()

    expected = (resource_dir_path / "file" / "expected.txt").read_text(encoding="utf-8")
    assert capfd.readouterr().out == expected


def test_genpnize_from_stdin():
    resource_dir_path = Path(__file__).parent / "resources"
    input_path = resource_dir_path / "stdin" / "input.txt"

    script_name = "genpnize.exe" if sys.platform == "win32" else "genpnize"
    script_path = Path(sysconfig.get_path("scripts")) / script_name
    genpnize_process = subprocess.run(
        [str(script_path), "-"],
        input=input_path.read_text(encoding="utf-8").replace("\n", ""),
        text=True,
        encoding="utf-8",
        check=True,
        capture_output=True,
    )

    expected = (resource_dir_path / "stdin" / "expected.txt").read_text(encoding="utf-8")
    assert genpnize_process.stdout == expected
