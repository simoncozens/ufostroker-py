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

This may also be used as a fontmake command line plugin:

```
fontmake --filter 'ufostroker::StrokeFilter(pre=True,Width=50)' -u OpenPaths.ufo -o ttf
```

Or by adding a lib key into the UFO file's `lib.plist` file:

```xml
    <key>com.github.googlei18n.ufo2ft.filters</key>
    <array>
      <dict>
        <key>name</key>
        <string>ufostroker.StrokeFilter</string>
        <key>pre</key>
        <true/>
        <key>kwargs</key>
        <dict>
            <key>Width</key>
            <integer>50</integer>
            <key>StartCap</key>
            <string>square</string>
            <key>EndCap</key>
            <string>square</string>
            <key>JoinType</key>
            <string>mitre</string>
            <key>RemoveExternal</key>
            <true/>
        </dict>
      </dict>
    </array>
```

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
