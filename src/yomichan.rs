use std::fmt::Write;

use crate::word_frequency::parser::WordFrequency;
use crate::word_frequency::stats::get_popularity;

pub const MAX_TERM_PER_BANK: u16 = 10000;

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

enum Identifier {
    Ichidan,
    Godan,
}

impl Identifier {
    fn to_identifier(s: &str) -> &str {
        match s {
            "v1" | "v1-s" => "v1",
            "v5aru" | "v5b" | "v5g" | "v5k" | "v5k-s" | "v5m" | "v5n" | "v5r" | "v5r-i" | "v5s"
            | "v5t" | "v5u" | "v5u-s" | "v5uru" => "v5",
            _ => "",
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
    sequence_number: u32,
}

impl Definition {
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
    pub fn sequence_number(&mut self, sequence_number: u32) -> &mut Self {
        self.sequence_number = sequence_number;
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
    fn pos_to_identifier(&self) -> String {
        let mut ret = String::new();
        if self.pos.is_empty() {
            ret = "".to_string();
        }
        for i in &self.pos[0] {
            match i.as_str() {
                "v1" | "v1-s" => {
                    ret = "v1".to_string();
                }
                "v5aru" | "v5b" | "v5g" | "v5k" | "v5k-s" | "v5m" | "v5n" | "v5r" | "v5r-i"
                | "v5s" | "v5t" | "v5u" | "v5u-s" | "v5uru" => {
                    ret = "v5".to_string();
                }
                _ => (),
            }
        }
        ret
    }

    pub fn serialize(&self, term_num: u16, vec_word_freq: &Vec<WordFrequency>) -> String {
        let mut plus_count: u16 = 0;
        let mut use_reading = false;
        let no_kanji_term = false;
        if self.reading.len() > self.term.len() {
            use_reading = true;
        }
        let len = if use_reading {
            self.reading.len()
        } else {
            self.term.len()
        };
        let mut ret = if term_num == 1 {
            "[".to_string()
        } else {
            String::new()
        };

        if no_kanji_term {
            for i in 0..len {
                write!(
                    ret,
                    r#"["{}","{}","","{}",{},["{}"],{},""]"#,
                    self.term[i],
                    self.reading[0],
                    self.pos_to_identifier(),
                    get_popularity(self.sequence_number, vec_word_freq) - i as f32,
                    self.serialize_gloss(),
                    term_num + plus_count,
                )
                .expect("Could not write to buffer string to serialize definitions");
                if term_num < MAX_TERM_PER_BANK {
                    ret = format!("{},\n", ret);
                } else {
                    write!(ret, "]")
                        .expect("Could not write to buffer string to serialize definitions");
                }

                plus_count += 1;
            }
        } else if use_reading {
            let len_term = self.term.len();
            for i in 0..len {
                for j in 0..len_term {
                    write!(
                        ret,
                        r#"["{}","{}","","{}",{},["{}"],{},""]"#,
                        self.term[j],
                        self.reading[i],
                        self.pos_to_identifier(),
                        get_popularity(self.sequence_number, vec_word_freq) - i as f32,
                        self.serialize_gloss(),
                        term_num + plus_count,
                    )
                    .expect("Could not write to buffer string to serialize definitions");
                    if term_num < MAX_TERM_PER_BANK {
                        ret = format!("{},\n", ret);
                    } else {
                        write!(ret, "]")
                            .expect("Could not write to buffer string to serialize definitions");
                    }

                    plus_count += 1;
                }
            }
        } else {
            let len_reading = self.reading.len();
            for i in 0..len {
                for j in 0..len_reading {
                    write!(
                        ret,
                        r#"["{}","{}","","{}",{},["{}"],{},""]"#,
                        self.term[i],
                        self.reading[j],
                        self.pos_to_identifier(),
                        get_popularity(self.sequence_number, vec_word_freq) - i as f32,
                        self.serialize_gloss(),
                        term_num + plus_count,
                    )
                    .expect("Could not write to buffer string to serialize definitions");
                    if term_num < MAX_TERM_PER_BANK {
                        ret = format!("{},\n", ret);
                    } else {
                        write!(ret, "]")
                            .expect("Could not write to buffer string to serialize definitions");
                    }

                    plus_count += 1;
                }
            }
        }

        // if uk then the len of term and reading is definitely > 0 so we don't have to check for it
        if self.uk {
            let len_reading = self.reading.len();
            for i in 0..len_reading {
                write!(
                    ret,
                    r#"["{}","","","{}",{},["{}"],{},""]"#,
                    self.reading[i],
                    self.pos_to_identifier(),
                    get_popularity(self.sequence_number, vec_word_freq) + 1f32,
                    self.serialize_gloss(),
                    term_num + plus_count,
                )
                .expect("Could not write to buffer string to serialize definitions");
                if term_num < MAX_TERM_PER_BANK {
                    ret = format!("{},\n", ret);
                } else {
                    write!(ret, "]")
                        .expect("Could not write to buffer string to serialize definitions");
                }

                plus_count += 1;
            }
        }
        ret
    }

    //["明白","めいはく","","",708,["めいはく【明白】\n〘adj-na〙\nobvious; clear; plain; evident; apparent; explicit; overt."],26,""],
    fn serialize_gloss(&self) -> String {
        let mut ret = String::new();
        write!(ret, "{}", self.reading.join("・")).unwrap();
        write!(ret, "【{}】", self.term.join("・")).unwrap();
        if self.pos.len() >= 1 {
            write!(ret, "\\n〘{}〙", self.pos[0].join("・")).unwrap();
        }
        if self.pos.len() == 1 {
            if !self.misc.is_empty() {
                write!(ret, "\\n〘{}〙", self.misc[0].join("・")).unwrap();
            }
            write!(ret, "\\n{}.", self.gloss[0].join("; ")).unwrap();
        } else {
            for (i, _val) in self.pos.iter().enumerate() {
                write!(
                    ret,
                    "\\n{} 〘{}〙 {}.",
                    i + 1,
                    self.misc[i].join("・"),
                    self.gloss[i].join("; ")
                )
                .unwrap();
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::word_frequency::parser::parse_frequency_input;

    #[test]
    #[ignore]
    fn serialize_single_term() {
        let raw_freq_input =
            std::fs::read_to_string("japanese-word-frequency/frequency.txt").unwrap();
        let (_, vec_word_freq) = parse_frequency_input(&raw_freq_input.as_bytes()).unwrap();

        let mut definitions = Definition::default();
        definitions.sequence_number(1000220u32);
        definitions.add_term("明白".to_string());
        definitions.add_reading("めいはく".to_string());
        definitions.add_pos("adj-na".to_string(), 1);
        definitions.add_gloss("obvious".to_string(), 1);
        definitions.add_gloss("clear".to_string(), 1);
        definitions.add_gloss("plain".to_string(), 1);
        definitions.add_gloss("evident".to_string(), 1);
        definitions.add_gloss("apparent".to_string(), 1);
        definitions.add_gloss("explicit".to_string(), 1);
        definitions.add_gloss("overt".to_string(), 1);

        let serialized = r#"["明白","めいはく","","",98,["めいはく【明白】\n〘adj-na〙\nobvious; clear; plain; evident; apparent; explicit; overt."],26,""],"#.to_string();
        let serialized = format!("{}\n", serialized);
        assert_eq!(definitions.serialize(26, &vec_word_freq), serialized);
    }

    #[test]
    #[ignore]
    fn serialize_multiple_term_uk() {
        let raw_freq_input =
            std::fs::read_to_string("japanese-word-frequency/frequency.txt").unwrap();
        let (_, vec_word_freq) = parse_frequency_input(&raw_freq_input.as_bytes()).unwrap();

        let mut definitions = Definition::default();
        definitions.sequence_number(1000300u32);
        definitions.set_uk();
        definitions.add_term("遇う".to_string());
        definitions.add_term("配う".to_string());
        definitions.add_reading("あしらう".to_string());
        definitions.add_pos("v5u".to_string(), 1);
        definitions.add_pos("vt".to_string(), 1);
        definitions.add_misc("uk".to_string(), 1);
        definitions.add_gloss("to treat".to_string(), 1);
        definitions.add_gloss("to handle".to_string(), 1);
        definitions.add_gloss("to deal with".to_string(), 1);
        definitions.add_pos("v5u".to_string(), 2);
        definitions.add_pos("vt".to_string(), 2);
        definitions.add_misc("uk".to_string(), 2);
        definitions.add_gloss("to arrange".to_string(), 2);
        definitions.add_gloss("to decorate".to_string(), 2);
        definitions.add_gloss("to dress".to_string(), 2);
        definitions.add_gloss("to garnish".to_string(), 2);

        let serialized_1 = r#"["遇う","あしらう","","v5",52,["あしらう【遇う・配う】\n〘v5u・vt〙\n1 〘uk〙 to treat; to handle; to deal with.\n2 〘uk〙 to arrange; to decorate; to dress; to garnish."],35,""],"#.to_string();
        let serialized = format!("{}", serialized_1);
        let serialized_2 = r#"["配う","あしらう","","v5",51,["あしらう【遇う・配う】\n〘v5u・vt〙\n1 〘uk〙 to treat; to handle; to deal with.\n2 〘uk〙 to arrange; to decorate; to dress; to garnish."],36,""],"#.to_string();
        let serialized = format!("{}\n{}", serialized, serialized_2);
        let serialized_3 = r#"["あしらう","","","v5",53,["あしらう【遇う・配う】\n〘v5u・vt〙\n1 〘uk〙 to treat; to handle; to deal with.\n2 〘uk〙 to arrange; to decorate; to dress; to garnish."],37,""],"#.to_string();
        let serialized = format!("{}\n{}\n", serialized, serialized_3);
        assert_eq!(definitions.serialize(35, &vec_word_freq), serialized);
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
