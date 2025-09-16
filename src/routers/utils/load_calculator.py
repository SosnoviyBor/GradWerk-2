import numpy as np

from src.modeler.utils.consts import DistributionType

def calculate_load(deviation: float,
                   dist: str,
                   mean: float,
                   replica: int):
    SAMPLE_SIZE = 10000  # Number of samples for load estimation
    
    match(dist):
        case DistributionType.exponential:
            sample = np.random.exponential(mean, SAMPLE_SIZE)
        case DistributionType.normal:
            sample = np.random.normal(mean, deviation, SAMPLE_SIZE)
        case DistributionType.uniform:
            sample = np.random.uniform(mean - deviation, mean + deviation, SAMPLE_SIZE)
        case DistributionType.erlang:
            sample = np.random.gamma(deviation, mean, SAMPLE_SIZE)
        case DistributionType.constant | _:
            sample = [mean for _ in range(SAMPLE_SIZE)]
    
    load = np.mean(sample) / replica
    
    return load
