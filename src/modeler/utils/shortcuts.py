from random import choice
from sys import maxsize

from src.modeler.utils.consts import NextElementType
from src.modeler.components.element import Element

def out_act(e: Element) -> None:
    match(e.next_element_type):
        case NextElementType.random:
            next_element = choice(e.next_element_array)
            next_element.in_act()
            
        case NextElementType.round_robin:
            if e.round_robin_idx == len(e.next_element_array):
                e.round_robin_idx = 0
            next_element = e.next_element_array[e.round_robin_idx]
            next_element.in_act()
            e.round_robin_idx += 1
            
        case NextElementType.balanced:
            shortest_queue_id = 0
            shortest_queue = maxsize
            for i in range(len(e.next_element_array)):
                next_element = e.next_element_array[i]
                free_queue = next_element.queue - next_element.state
                if free_queue < shortest_queue:
                    shortest_queue_id = i
                    shortest_queue = free_queue
            next_element = e.next_element_array[shortest_queue_id]
            next_element.in_act()
            
        case _:
            raise(f"Recieved unknown next element type: {e.next_element_type}")