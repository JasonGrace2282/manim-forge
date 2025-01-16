import numpy as np
from manim import Camera

from . import cairo


def setup():
    def set_cairo_context_path(self, ctx, vmobject):
        points = self.transform_points_pre_display(vmobject, vmobject.points)
        if len(points) == 0:
            return
        cairo.CairoCamera().set_cairo_context_path(ctx, vmobject, np.asarray(points, dtype=np.float64))
        return self

    # Monkeypatch the method
    Camera.set_cairo_context_path = set_cairo_context_path

