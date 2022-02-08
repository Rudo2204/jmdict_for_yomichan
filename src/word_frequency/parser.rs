use nom::bytes::complete::{take, take_till, take_until, take_while};
use nom::character::complete::line_ending;
use nom::character::{is_newline, is_space};
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

use std::str::from_utf8;

#[derive(Debug)]
pub struct WordFrequency {
    pub ent_seq: u32,
    pub popularity: f32,
    pub term: String,
    pub reading: String,
}

fn consume_line(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    tuple((take_till(is_newline), line_ending))(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], WordFrequency> {
    let (freq, (line, _)) = consume_line(input)?;
    let (_, (ent_seq, _, popularity, _, term, _, reading)) = tuple((
        take_till(is_space),
        take_while(is_space),
        take_till(is_space),
        take_while(is_space),
        take_until("["),
        take(1u8),
        take_until("]"),
    ))(line)?;

    let ent_seq = from_utf8(ent_seq).unwrap().parse::<u32>().unwrap();
    let popularity = from_utf8(popularity).unwrap().parse::<f32>().unwrap();
    let term = from_utf8(term).unwrap().to_string();
    let reading = from_utf8(reading).unwrap().to_string();

    let word_frequency = WordFrequency {
        ent_seq,
        popularity,
        term,
        reading,
    };
    Ok((freq, word_frequency))
}

pub fn parse_frequency_input(input: &[u8]) -> IResult<&[u8], Vec<WordFrequency>> {
    let (_freq, (vec_word_freq, _)) = many_till(parse_line, eof)(input)?;
    Ok((_freq, vec_word_freq))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample() {
        let raw_freq_sample = std::fs::read_to_string("tests/frequency-sample.txt").unwrap();
        let (_, _vec_word_freq) = parse_frequency_input(raw_freq_sample.as_bytes()).unwrap();
    }

    #[test]
    #[ignore]
    fn parse_full_input() {
        let raw_freq_sample =
            std::fs::read_to_string("japanese-word-frequency/frequency.txt").unwrap();
        let (_, _vec_word_freq) = parse_frequency_input(raw_freq_sample.as_bytes()).unwrap();
    }
}
