# ufostroker-py

This Python module wraps the Rust [`MFEK/math`](https://github.com/MFEK/math.rlib)
library's capability for path stroking. It currently exports one function,
`constant_width_stroke`:

```python
from ufostroker import constant_width_stroke
f = ufoLib2.Font("tests/data/Scurve.ufo")

constant_width_stroke(f["A"], width=50, startcap="square", endcap="square")
```

This modifies the glyph in place. The library works with defcon and ufoLib2
objects.

## Building

Use `maturin` to build `ufostroker`.

```
pip3 install maturin
python3 -m venv strokervenv
. ./strokervenv/bin/activate
maturin develop
maturin build # Build wheel
```

## License

Apache 2.
