from typing import List
from sys import maxsize
from time import perf_counter
import psutil

from src.modeler.components.element import Element
from src.modeler.components.create import Create
from src.modeler.components.process import Process
from src.modeler.components.dispose import Dispose


class Model:
    def __init__(self, elements: List[Element]) -> None:
        self.elements = elements

        self.iteration = 0
        self.tnext = .0
        self.tcurr = self.tnext
        self.log = {
            "first": [f"There are {len(self.elements)} elements in the simulation"],
            "last": [],
        }
        
        self.process = psutil.Process()
        self.mem_peak = .0
        self.mem_total = .0
        self.mem_samples = 0
        self.start_time = perf_counter()
        self.last_sample = self.start_time
        self.sample_interval = .5 # in seconds

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
        timer_start = perf_counter()
        self._mainloop(time, log_max_size)
        time_elapsed = perf_counter() - timer_start

        self._log_sim_results()
        # trim trailing newline
        self.log["last"][0] = self.log["last"][0][1:]

        # prevent errors if the simulation was too fast
        if self.mem_samples == 0:
            self.sample_interval = 0
            self.update_mem_stats()

        return {
            "results": self._collect_sim_summary(),
            "log": self.log,
            "iterations": self.iteration,
            "sim_time": round(time_elapsed, 4),
            "total_time": .0,
            "iters_per_sec": round(self.iteration / time_elapsed, 4),
            # in MB
            "mem_peak": round(self.mem_peak / 1024, 4),
            "mem_mean": round(self.mem_total / self.mem_samples / 1024, 4),
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
            self.update_mem_stats()

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

    def update_mem_stats(self):
        now = perf_counter()
        if now - self.last_sample >= self.sample_interval:
            mem_curr = self.process.memory_info().rss / 1024 # to KB
            self.mem_peak = max(self.mem_peak, mem_curr)
            self.mem_total += mem_curr
            self.mem_samples += 1
            self.last_sample = now
