from typing import List
from sys import maxsize
from time import perf_counter
import psutil, os

from src.modeler.components.element import Element
from src.modeler.components.create import Create
from src.modeler.components.process import Process
from src.modeler.components.dispose import Dispose


class Model:
    def __init__(self, elements: List[Element]) -> None:
        self.elements = elements

        self.iteration = -1
        self.tnext = 0.0
        self.tcurr = self.tnext
        self.log = {"first": [], "last": []}

    def simulate(self, time: float, log_max_size: int) -> dict:
        """
        return: {
            results: dict[ check _collect_sim_summary() for details ],
            log: [str],
            time: float,
            memory: float,
            iterations: int,
        }
        """
        # reset model state
        self.iteration = 0
        self.log["first"] = [
            f"There are {len(self.elements)} elements in the simulation"
        ]
        self.log["last"] = []
        
        # init measurements
        process = psutil.Process(os.getpid())
        # initialize CPU percent measurement
        timer_start = perf_counter()
        
        self._mainloop(time, log_max_size)
        
        # finalize measurements
        timer_result = perf_counter() - timer_start
        memory_usage = process.memory_info().rss / (1024 * 1024) # in MB
        
        self._log_sim_results()
        # trim trailing newline
        self.log["last"][0] = self.log["last"][0][1:]
        return {
            "results": self._collect_sim_summary(),
            "log": self.log,
            "time": timer_result,
            "memory": memory_usage,
            "iterations": self.iteration
        }

    def _mainloop(self, time: float, log_max_size: int) -> None:
        # thats it
        # thats the whole algorithm for ya
        while self.tcurr < time:
            # searching nearest event
            self.tnext = maxsize
            event_id = 0
            for element in self.elements:
                if element.get_tnext() < self.tnext:
                    self.tnext = element.get_tnext()
                    event_id = element.id
            # update current time of each element + calculate some stats
            tcurr_old = self.tcurr
            self.tcurr = self.tnext
            for element in self.elements:
                element.do_statistics(self.tnext - tcurr_old)
                element.tcurr = self.tcurr
            # move things between relevant elements queues
            self.elements[event_id].out_act()
            for element in self.elements:
                if element.get_tnext() == self.tcurr:
                    element.out_act()
            # logging
            self.iteration += 1
            self._log_event(event_id, log_max_size)

    def _log_event(self, event_id: int, log_max_size: int) -> None:
        # generate message
        msg = (
            f"\n"
            f">>>     Event #{self.iteration} in {self.elements[event_id].name}    <<<\n"
            f">>>     time: {round(self.tnext, 4)}    <<<\n"
        )
        for element in self.elements:
            msg += element.get_summary()
        # update log
        if len(self.log["first"]) <= log_max_size:
            self.log["first"].append(msg)
        else:
            self.log["last"].append(msg)
            if len(self.log["last"]) > log_max_size:
                del self.log["last"][0]

    def _log_sim_results(self) -> None:
        msg = "\n-------------RESULTS-------------\n"

        for element in self.elements:
            msg += f"##### {element.name} #####\n" f"quantity = {element.quantity}\n"

            if isinstance(element, Process):
                failure_prob = 0
                if element.failure + element.quantity != 0:
                    failure_prob = element.failure / (
                        element.failure + element.quantity
                    )

                msg += (
                    f"Mean length of queue = {element.mean_queue / self.tcurr}\n"
                    f"Failure probability = {failure_prob}\n"
                )

            msg += "\n"

        msg = msg[:-1]
        msg += "---------------------------------\n" "Simulation is done successfully!"
        self.log["last"].append(msg)

    def _collect_sim_summary(self) -> list:
        """
        returns: [{
            data {
                id: int,
                name: str,
                !mean: float,
                !deviation: float,
                !worker_count: int,
                !distribution: int,
                !max_queue: int
            },
            result {
                quantity: int,
                !failures: int,
                !mean_queue_length: float,
                !failure_probability: float
            }
        }]
        """
        model_data = []
        for element in self.elements:
            element_data = {"id": element.id, "name": element.name}
            if isinstance(element, Create):
                element_data["mean"] = element.delay_mean
                element_data["worker_count"] = element.worker_count
                element_data["distribution"] = element.distribution
                element_data["deviation"] = element.delay_deviation

            elif isinstance(element, Process):
                element_data["mean"] = element.delay_mean
                element_data["worker_count"] = element.worker_count
                element_data["distribution"] = element.distribution
                element_data["deviation"] = element.delay_deviation
                element_data["max_queue"] = element.max_queue

            elif isinstance(element, Dispose):
                pass

            element_result = {"quantity": element.quantity}
            if isinstance(element, Create):
                pass

            if isinstance(element, Process):
                failure_prob = 0
                if element.failure + element.quantity != 0:
                    failure_prob = element.failure / (
                        element.failure + element.quantity
                    )

                element_result["failures"] = element.failure
                element_result["mean_queue_length"] = element.mean_queue / self.tcurr
                element_result["failure_probability"] = failure_prob

            elif isinstance(element, Dispose):
                pass

            model_data.append({"data": element_data, "result": element_result})

        return model_data
