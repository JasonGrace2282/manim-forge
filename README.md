# Manim-Forge
Some parts of the python library [Manim](https://www.manim.community) need
to be executed fast. This library contains code
for these computations, but written in Rust
to improve speed.

## Usage
First, [install manim](https://docs.manim.community/en/stable/installation.html).
After that, it should just be
```
pip install manimforge
```
In the off-chance your operating system doesn't have prebuilt wheels,
you'll need to [install Rust](https://www.rust-lang.org/tools/install).

After that, it should be as simple as inserting the following before
rendering a scene:
```py
import manimforge as mf
mf.setup()
```

### Supported Versions
This library is only tested with the following:

- Windows (x86-64)
- macOS (x86-64)
- macOS (aarch64)
- Linux (x86-64 glibc)
- Linux (x86 glibc)

This library supports the versions of python that Manim
itself supports. However, there are some exceptions: namely
that free-threaded builds of python are not supported.

## Testing
1. Clone the repo
2. Create a virtual environment using something like [`uv`](https://docs.astral.sh/uv/) (`uv venv`)
3. Install dev dependencies (`uv sync`)
4. Build the library (`uv run maturin develop --uv`)
5. Run the example (`uv run manim -p examples/circle.py --disable_caching`)
