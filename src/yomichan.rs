pub struct DictIndex {
    title: String,
    format: u8,
    revision: String,
    sequenced: bool,
}

impl DictIndex {
    fn serialize(&self) -> String {
        format!(
            r#"{{"title":"{}","format":{},"revision":"{}","sequenced":{}}}"#,
            self.title, self.format, self.revision, self.sequenced
        )
    }
    pub fn default_serialize() -> String {
        let revision_time_format = time::format_description::parse("[year][month][day]")
            .expect("Could not parse to YYYYMMDD");

        let revision_date = time::OffsetDateTime::now_local()
            .expect("Could not get local time")
            .format(&revision_time_format)
            .expect("Could not parse to YYYYMMDD");

        let dict_index = DictIndex {
            title: "JMdict".to_string(),
            format: 3u8,
            revision: format!("JMdict-{}", revision_date),
            sequenced: true,
        };

        dict_index.serialize()
    }
}

struct Term {
    term: String,
    reading: String,
    //tags: Option<String>,
    // If noun, return None
    identifiers: Identifier,
    popularity: f32,
    definitions: String,
    sequence_number: u32,
    //extra_tags: Option<String>,
}

impl Term {
    fn serialize(&self) -> String {
        format!(
            r#"["{}","{}","","{}",{},["{}"],{},""]"#,
            self.term,
            self.reading,
            self.identifiers.as_str(),
            self.popularity,
            self.definitions,
            self.sequence_number
        )
    }
}

enum Identifier {
    Ichidan,
    Godan,
    Other,
}

impl Identifier {
    fn as_str(&self) -> &'static str {
        match self {
            Identifier::Ichidan => "v1",
            Identifier::Godan => "v5",
            Identifier::Other => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word_frequency::parser::parse_frequency_input;

    #[test]
    fn get_popularity_sample() {
        let raw_freq_sample = std::fs::read_to_string("tests/frequency-sample.txt").unwrap();
        let (_, vec_word_freq) = parse_frequency_input(raw_freq_sample.as_bytes()).unwrap();
        assert_eq!(
            get_popularity(&vec_word_freq, 1000310u32).unwrap(),
            36.9_f32
        );
        assert_eq!(
            get_popularity(&vec_word_freq, 1000225u32).unwrap(),
            36.9_f32
        );
        assert_eq!(get_popularity(&vec_word_freq, 1000300u32).unwrap(), 52_f32);
    }

    #[test]
    fn serialize_single_term() {
        let term = Term {term: "明白".to_string(), reading: "めいはく".to_string(), identifiers: Identifier::Other, popularity: 708f32, definitions: r#"めいはく【明白】\n〘adj-na〙\nobvious; clear; plain; evident; apparent; explicit; overt."#.to_string(), sequence_number: 26u32};
        let serialized = r#"["明白","めいはく","","",708,["めいはく【明白】\n〘adj-na〙\nobvious; clear; plain; evident; apparent; explicit; overt."],26,""]"#.to_string();
        assert_eq!(term.serialize(), serialized);
    }

    #[test]
    fn serialize_dict_index() {
        let dict_index = DictIndex {
            title: "JMdict".to_string(),
            format: 3u8,
            revision: "JMdict1".to_string(),
            sequenced: true,
        };
        assert_eq!(
            dict_index.serialize(),
            r#"{"title":"JMdict","format":3,"revision":"JMdict1","sequenced":true}"#
        );
    }
}
