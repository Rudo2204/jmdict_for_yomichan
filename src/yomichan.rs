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
    definitions: Definition,
    sequence_number: u32,
    //extra_tags: Option<String>,
}

//impl Term {
//    fn serialize(&self) -> String {
//        format!(
//            r#"["{}","{}","","{}",{},["{}"],{},""]"#,
//            self.term,
//            self.reading,
//            self.identifiers.as_str(),
//            self.popularity,
//            self.definitions,
//            self.sequence_number
//        )
//    }
//}

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

#[derive(Debug, Default)]
pub struct Definition {
    // <keb> AKA multiple kanji form/ or multiple readings
    term: Vec<String>,
    // <reb>
    reading: Vec<String>,
    pos: Vec<Vec<String>>,
    uk: bool,
    gloss: Vec<Vec<String>>,
    misc: Vec<Vec<String>>,
}

impl Definition {
    fn serialize(&self) -> String {
        "".to_string()
    }
    pub fn add_term(&mut self, term: String) -> &mut Self {
        self.term.push(term);
        self
    }
    pub fn add_reading(&mut self, reading: String) -> &mut Self {
        self.reading.push(reading);
        self
    }
    pub fn add_misc(&mut self, misc: String, sense: usize) -> &mut Self {
        if self.misc.is_empty() {
            self.misc = vec![vec![misc]];
        } else if self.misc.len() == sense {
            let mut sense = self.misc.pop().unwrap();
            sense.push(misc);
            self.misc.push(sense);
        } else if self.misc.len() < sense {
            let sense = vec![misc];
            self.misc.push(sense);
        }
        self
    }
    pub fn add_pos(&mut self, pos: String, sense: usize) -> &mut Self {
        if self.pos.is_empty() {
            self.pos = vec![vec![pos]];
        } else if self.pos.len() == sense {
            let mut sense = self.pos.pop().unwrap();
            sense.push(pos);
            self.pos.push(sense);
        } else if self.pos.len() < sense {
            let sense = vec![pos];
            self.pos.push(sense);
        }
        self
    }
    pub fn set_uk(&mut self) -> &mut Self {
        self.uk = true;
        self
    }
    pub fn add_gloss(&mut self, gloss: String, sense: usize) -> &mut Self {
        if self.gloss.is_empty() {
            self.gloss = vec![vec![gloss]];
        } else if self.gloss.len() == sense {
            let mut sense = self.gloss.pop().unwrap();
            sense.push(gloss);
            self.gloss.push(sense);
        } else if self.gloss.len() < sense {
            let sense = vec![gloss];
            self.gloss.push(sense);
        }
        self
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
