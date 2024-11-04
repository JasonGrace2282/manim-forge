# from manimforge.cairo import CairoCamera
import manimforge
from manim import *

cairoCamera = manimforge.cairo.CairoCamera()

class MyCamera(Camera):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

    def set_cairo_context_path(self, ctx, vmobject: VMobject):
        cairoCamera.set_cairo_context_path(ctx, vmobject)

class MyMovingCamera(MovingCamera, MyCamera):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

class Test(MovingCameraScene):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(
            camera_class=MyMovingCamera,
            *args,
            **kwargs
        )

    def construct(self) -> None:
        self.camera.frame.save_state()

        # Create the axes and the curve
        ax = Axes(x_range=[-1, 10], y_range=[-1, 10])
        graph = ax.plot(lambda x: np.sin(x), color=BLUE, x_range=[0, 3*PI])

        # Create Dots based on the graph
        moving_dot = Dot(ax.i2gp(graph.t_min, graph), color=ORANGE)
        dot_1 = Dot(ax.i2gp(graph.t_min, graph))
        dot_2 = Dot(ax.i2gp(graph.t_max, graph))

        self.add(ax, graph, dot_1, dot_2, moving_dot)
        self.play(self.camera.frame.animate.scale(0.5).move_to(moving_dot))

        def update_curve(mob):
            mob.move_to(moving_dot.get_center())

        self.camera.frame.add_updater(update_curve)
        self.play(MoveAlongPath(moving_dot, graph, rate_func=linear))
        self.camera.frame.remove_updater(update_curve)

        self.play(Restore(self.camera.frame))

