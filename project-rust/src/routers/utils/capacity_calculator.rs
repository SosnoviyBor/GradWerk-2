use crate::modeler::utils::consts::DistributionType;
use crate::modeler::utils::random;

const SAMPLE_SIZE: u32 = 10000;

pub fn calculate_capacity(deviation: f64, dist: DistributionType, mean: f64, replica: u32) -> f64 {
    match dist {
        DistributionType::Exponential => {
            streaming_mean((0..SAMPLE_SIZE).map(|_| random::exponential(mean))) / replica as f64
        }
        DistributionType::Normal => {
            streaming_mean((0..SAMPLE_SIZE).map(|_| random::normal(mean, deviation)))
                / replica as f64
        }
        DistributionType::Uniform => {
            streaming_mean(
                (0..SAMPLE_SIZE).map(|_| random::uniform(mean - deviation, mean + deviation)),
            ) / replica as f64
        }
        DistributionType::Erlang => {
            streaming_mean((0..SAMPLE_SIZE).map(|_| random::erlang(mean, deviation as usize)))
                / replica as f64
        }
        DistributionType::Constant => return mean,
    }
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
