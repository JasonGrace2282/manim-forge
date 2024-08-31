from typing import Any, Self

import numpy as np
import numpy.typing as npt
import cairo

class CairoCamera:
    def __init__(self, *args, **kwargs) -> None: ...

    def set_cairo_context_path(self, ctx: cairo.Context, vmobject: Any, points: npt.NDArray[np.float64]) -> None:
        """Traces a VMobject on the :class:`cairo.Context`.

        Parameters
        ----------
            ctx: The pycairo context.
            vmobject: A Manim Community VMobject
            points: A numpy array of floats
        """

    def __copy__(self) -> Self: ...

    def __deepcopy__(self, memo: Any) -> Self: ...
