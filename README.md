# Manim-Forge
Some parts of the python library [Manim](https://www.manim.community) need
to be executed fast. This library contains code
for these computations, but written in Rust
to improve speed.

## Testing
1. Clone the repo
2. Create a virtual environment using something like [`uv`](https://docs.astral.sh/uv/) (`uv venv`)
3. Install dev dependencies (`uv sync`)
4. Build the library (`uv run maturin develop --uv`)
5. Run the example (`uv run manim -p examples/circle.py --disable_caching`)
