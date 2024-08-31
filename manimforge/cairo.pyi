from typing import Any, Self

import numpy as np
import numpy.typing as npt
import cairo

class CairoCamera:
    def __init__(self, *args, **kwargs) -> None: ...

    def set_cairo_context_path(self, ctx: cairo.Context, vmobject: Any, points: npt.NDArray[np.float64]) -> None: ...

    def __copy__(self) -> Self: ...

    def __deepcopy__(self, memo: Any) -> Self: ...
