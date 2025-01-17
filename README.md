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
If you're using a mainstream operating system and this happens (macOS,
Windows, some glibc-based linux distros), please file a bug!

After that, it should be as simple as inserting the following before
rendering a scene:
```py
import manimforge as mf
mf.setup()
```

## Testing
1. Clone the repo
2. Create a virtual environment using something like [`uv`](https://docs.astral.sh/uv/) (`uv venv`)
3. Install dev dependencies (`uv sync`)
4. Build the library (`uv run maturin develop --uv`)
5. Run the example (`uv run manim -p examples/circle.py --disable_caching`)
