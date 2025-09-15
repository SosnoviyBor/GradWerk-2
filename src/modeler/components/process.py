from sys import maxsize

from src.modeler.components.element import Element
from src.modeler.utils import shortcuts


class Process(Element):
    def __init__(self, delay: float, worker_count: int) -> None:
        super().__init__(delay, worker_count)
        self.max_queue = maxsize
        self.mean_queue = .0
        self.wait_start = .0
        self.wait_time = .0
        self.failure = 0
        self.state_sum = 0
        self.put_tnext(maxsize)
    
    
    def in_act(self) -> None:
        if self.state < self.worker_count:
            self.state += 1
            self.put_tnext(self.tcurr + self.get_delay())
            if self.state == 0:
                self.wait_time += self.tcurr - self.wait_start
        else:
            if self.queue < self.max_queue:
                self.queue += 1
            else:
                self.failure += 1
    
    
    def out_act(self) -> None:
        super().out_act()
        self.state -= 1
        if self.queue > 0:
            self.queue -= 1
            self.state += 1
            self.put_tnext(self.tcurr + self.get_delay())
        else:
            self.wait_start = self.tcurr
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
                f"tnext: {nearest_tnext} | "
                f"average_load: {round(self.average_load, 4)}\n"
                
                f"failure: {self.failure}")
    
    
    def do_statistics(self, delta: float) -> None:
        self.state_sum += delta * self.state
        self.mean_queue += self.queue * delta