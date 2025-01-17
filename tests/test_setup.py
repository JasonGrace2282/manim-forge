import manimforge as mf
from manim import Camera


def test_setup_works():
    old_ctx_method = Camera.set_cairo_context_path
    mf.setup()
    # this method should have been monkeypatched
    assert Camera.set_cairo_context_path != old_ctx_method
