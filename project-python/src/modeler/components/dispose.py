from sys import maxsize

from src.modeler.components.element import Element


class Dispose(Element):
    def __init__(self) -> None:
        super().__init__(0, 0)
        self.put_tnext(maxsize)

    def in_act(self) -> None:
        self.out_act()

    def out_act(self) -> None:
        super().out_act()

    def get_summary(self) -> str:
        nearest_tnext = self.get_tnext()
        if nearest_tnext != maxsize:
            self.average_load = self.quantity / nearest_tnext

        return f"\n##### {self.name} #####\n" f"quantity: {self.quantity}"
