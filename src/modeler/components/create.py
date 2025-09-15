from sys import maxsize

from src.modeler.components.element import Element
from src.modeler.utils import shortcuts

class Create(Element):
    def __init__(self, delay: float, worker_count: int) -> None:
        super().__init__(delay, worker_count)
        self.put_tnext(.00001)
    
    
    def out_act(self) -> None:
        super().out_act()
        self.put_tnext(self.tcurr + self.get_delay())
        shortcuts.out_act(self)
        self.pop_tnext_from_queue()
    
    
    def get_summary(self) -> str:
        nearest_tnext = self.get_tnext()
        if nearest_tnext != maxsize:
            self.average_load = self.quantity / nearest_tnext
            nearest_tnext = round(nearest_tnext, 4)
        else:
            nearest_tnext = "maxval"

        return (f"\n##### {self.name} #####\n"
                f"state: {self.state} | "
                f"quantity: {self.quantity} | "
                f"queue: {self.queue} | "
                f"tnext: {round(nearest_tnext, 4)} | "
                f"average_load: {round(self.average_load, 4)}")