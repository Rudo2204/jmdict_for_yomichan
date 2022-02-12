use crate::word_frequency::parser::*;

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
pub fn get_popularity(ent_seq: u32, vec_word_freq: &Vec<WordFrequency>) -> f32 {
    let (mean, std_deviation) = get_freq_stats(&vec_word_freq);
    let default_mean = mean - std_deviation;
    match vec_word_freq.into_iter().find(|&x| x.ent_seq == ent_seq) {
        Some(word_freq) => word_freq.popularity,
        None => default_mean,
    }
}

mod tests {
    use super::*;
    use crate::word_frequency::parser::parse_frequency_input;

    #[test]
    fn get_popularity_sample() {
        let raw_freq_sample = std::fs::read_to_string("tests/frequency-sample.txt").unwrap();
        let (_, vec_word_freq) = parse_frequency_input(raw_freq_sample.as_bytes()).unwrap();
        assert_eq!(get_popularity(1000310u32, &vec_word_freq), 36.9_f32);
        assert_eq!(get_popularity(1000225u32, &vec_word_freq), 36.9_f32);
        assert_eq!(get_popularity(1000300u32, &vec_word_freq), 52_f32);
    }
}
