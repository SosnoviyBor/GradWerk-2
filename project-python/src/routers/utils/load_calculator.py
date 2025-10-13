import numpy as np

from src.modeler.utils.consts import DistributionType
import src.utils.rng as rng


def calculate_load(deviation: float, dist: str, mean: float, replica: int):
    SAMPLE_SIZE = 10000  # Number of samples for load estimation

    match (dist):
        case DistributionType.exponential:
            sample = rng.exp(mean, SAMPLE_SIZE)
        case DistributionType.normal:
            sample = rng.normal(mean, deviation, SAMPLE_SIZE)
        case DistributionType.uniform:
            sample = rng.uniform(mean - deviation, mean + deviation, SAMPLE_SIZE)
        case DistributionType.erlang:
            sample = rng.erlang(mean, deviation, SAMPLE_SIZE)
        case DistributionType.constant | _:
            return mean

    return np.mean(sample) / replica
