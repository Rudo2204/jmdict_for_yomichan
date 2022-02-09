use crate::word_frequency::parser::WordFrequency;

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

// this returns the first (which is also the highest) popularity for an input ent_seq
// for other terms with the same ent_seq, we will simply reduce this popularity point
pub fn get_popularity(vec_word_freq: &Vec<WordFrequency>, ent_seq: u32) -> Option<f32> {
    match vec_word_freq.into_iter().find(|&x| x.ent_seq == ent_seq) {
        Some(word_freq) => Some(word_freq.popularity),
        None => None,
    }
}
