use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::modeler::utils::consts::DistributionType;
use crate::modeler::utils::random;

const SAMPLE_SIZE: u32 = 10000;

pub fn calculate_capacity(deviation: f64, dist: DistributionType, mean: f64, replica: u32) -> f64 {
    if dist == DistributionType::Constant {
        return replica as f64 / mean;
    }

    let mut rng = SmallRng::seed_from_u64(0);

    let iter = (0..SAMPLE_SIZE).map(|_| match dist {
        DistributionType::Exponential => random::exponential(&mut rng, mean),
        DistributionType::Normal => random::normal(&mut rng, mean, deviation),
        DistributionType::Uniform => random::uniform(&mut rng, mean - deviation, mean + deviation),
        DistributionType::Erlang => random::erlang(&mut rng, mean, deviation as usize),
        DistributionType::Constant => unreachable!(),
    });

    // average time per item (in seconds)
    let avg_time = streaming_mean(iter);

    replica as f64 / avg_time
}

fn streaming_mean<I>(iter: I) -> f64
where
    I: IntoIterator<Item = f64>,
{
    let mut sum = 0.0;
    let mut count = 0usize;
    for val in iter {
        sum += val;
        count += 1;
    }
    sum / count as f64
}
