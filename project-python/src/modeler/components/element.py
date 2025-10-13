from queue import PriorityQueue

from src.modeler.utils.consts import DistributionType, NextElementType
import src.utils.rng as rng


class Element:
    next_id = 0

    def __init__(self, delay: float, worker_count: int) -> None:
        # FUCK PRIVATE VARIABLES, SETTERS AND GETTERS
        # I KNOW WHAT IM DOING YOU MORON
        # THE FILE NOW IS 3 GODDAMN TIMES SMALLER NERD
        self.worker_count = worker_count
        self.delay_mean = delay

        self.id = Element.next_id
        Element.next_id += 1
        self.name = f"Element {self.id}"

        self.tnext = PriorityQueue()
        self.delay_deviation = 0.0
        self.k = 0  # used only in erlang()
        self.distribution = DistributionType.exponential
        self.next_element_type = NextElementType.random
        self.tcurr = 0.0
        self.state = 0
        self.queue = 0  # amount of tnexts
        self.quantity = 0
        self.average_load = 0
        self.round_robin_idx = 0

        self.next_element_array: list

    @staticmethod
    def refresh_next_id() -> None:
        Element.next_id = 0

    def get_delay(self) -> float:
        match (self.distribution):
            case DistributionType.exponential:
                return rng.exp(self.delay_mean)
            case DistributionType.normal:
                return rng.normal(self.delay_mean, self.delay_deviation)
            case DistributionType.uniform:
                return rng.uniform(
                    self.delay_mean - self.delay_deviation,
                    self.delay_mean + self.delay_deviation,
                )
            case DistributionType.erlang:
                return rng.gamma(self.k, self.delay_mean)
            case DistributionType.constant | _:
                return self.delay_mean

    def get_tnext(self) -> float:
        return self.tnext.queue[0]

    def set_next_element_balanced(self, next_element_array: list) -> None:
        self.next_element_type = NextElementType.balanced
        self.next_element_array = next_element_array

    def set_next_element_roundrobin(self, next_element_array: list) -> None:
        self.next_element_type = NextElementType.round_robin
        self.next_element_array = next_element_array

    def set_next_element_random(self, next_element_array: list) -> None:
        self.next_element_type = NextElementType.random
        self.next_element_array = next_element_array

    def in_act(self) -> None:
        pass

    def out_act(self) -> None:
        self.quantity += 1

    def do_statistics(self, delta: float) -> None:
        pass

    def put_tnext(self, tnext: float) -> None:
        self.tnext.put(tnext)

    def pop_tnext_from_queue(self) -> float:
        return self.tnext.get()

    def get_summary(self) -> str:
        pass
