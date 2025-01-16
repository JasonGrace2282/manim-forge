import manimforge as mf
from manim import *


mf.setup()

# Example Scene
class Test(Scene):
    def construct(self):
        circle = Circle()
        self.play(Create(circle))

if __name__ == "__main__":
    config.preview = True
    config.disable_caching = True
    Test().render()
