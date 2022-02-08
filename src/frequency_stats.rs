use crate::frequency_parser::WordFrequency;

fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}
fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

pub fn get_freq_stats(vec_word_freq: &Vec<WordFrequency>) -> (f32, f32) {
    let data = vec_word_freq
        .iter()
        .map(|x| x.popularity)
        .collect::<Vec<f32>>();
    let data_mean = mean(&data).unwrap();
    let data_std_deviation = std_deviation(&data).unwrap();
    (data_mean, data_std_deviation)
}
