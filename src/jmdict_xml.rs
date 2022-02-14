use anyhow::Result;
use log::debug;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::bytes::Regex;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::str;

use crate::word_frequency::parser::WordFrequency;
use crate::yomichan::Definition;
use crate::yomichan::MAX_TERM_PER_BANK;

pub fn process_jmdict(xml: &str, vec_word_freq: &Vec<WordFrequency>) -> Result<()> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut current_term_count = 0;
    let mut buf = Vec::new();
    //let mut definition = String::new();

    let entity_re = Regex::new(r#"<!ENTITY\s+([^ \t\r\n]+)\s+"([^"]*)"\s*>"#)?;
    let mut custom_entities = HashMap::new();

    let mut current_term_bank_count: u8 = 1;
    let mut current_term_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(format!("term_bank_{}.json", current_term_bank_count))?;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"entry" => {
                    current_term_count += 1;

                    if current_term_count == MAX_TERM_PER_BANK {
                        //current_term_file.flush()?;
                        //current_term_file = OpenOptions::new()
                        //    .create(true)
                        //    .write(true)
                        //    .truncate(true)
                        //    .open(format!("term_bank_{}.json", current_term_bank_count))?;

                        current_term_count = 1;
                    }

                    let definition = parse_entry(&mut reader, &mut buf, &custom_entities)?;
                    debug!("{:#?}", definition);
                    write!(
                        current_term_file,
                        "{}",
                        definition.serialize(current_term_count, vec_word_freq)
                    )
                    .unwrap();
                }
                _ => (),
            },
            Ok(Event::DocType(ref e)) => {
                for cap in entity_re.captures_iter(&e) {
                    custom_entities.insert(cap[1].to_vec(), cap[1].to_vec());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

        buf.clear();
    }
    //println!("count = {}", count);
    //println!("txt = {:#?}", txt);
    Ok(())
}

fn parse_entry<R: BufRead>(
    reader: &mut Reader<R>,
    buf: &mut Vec<u8>,
    custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
) -> Result<Definition> {
    let mut definition = Definition::default();
    let mut number_of_sense: usize = 0;
    let mut current_tag = Tag::OtherDontCareAbout;

    loop {
        match reader.read_event(buf)? {
            // a tag was opened
            Event::Start(start) => {
                current_tag = Tag::from_str(str::from_utf8(start.name())?);
                if current_tag == Tag::Sense {
                    number_of_sense += 1;
                }
            }
            Event::Text(text) => {
                let value =
                    text.unescape_and_decode_with_custom_entities(&reader, &custom_entities)?;
                match current_tag {
                    Tag::EntSeq => {
                        definition.sequence_number(
                            value
                                .parse::<u32>()
                                .expect("could not parse sequence_number to u32"),
                        );
                    }
                    Tag::Keb => {
                        definition.add_term(value);
                    }
                    Tag::Reb => {
                        definition.add_reading(value);
                    }
                    Tag::Pos => {
                        definition.add_pos(value, number_of_sense);
                    }
                    Tag::Misc => {
                        match value.as_str() {
                            "uk" => {
                                definition.set_uk();
                            }
                            _ => (),
                        };

                        definition.add_misc(value, number_of_sense);
                    }
                    Tag::Gloss => {
                        definition.add_gloss(value, number_of_sense);
                    }
                    Tag::Sense => {
                        number_of_sense += 1;
                    }
                    Tag::OtherDontCareAbout => (),
                }
            }
            Event::Empty(_val) => {
                definition.set_uk();
            }
            Event::End(end) => {
                if end.name() == b"entry" {
                    break;
                }
            }
            _ => (),
        }
    }
    Ok(definition)
}

#[derive(PartialEq)]
enum Tag {
    EntSeq,
    Keb,
    Reb,
    Pos,
    Gloss,
    Misc,
    Sense,
    OtherDontCareAbout,
}

impl Tag {
    fn from_str(s: &str) -> Self {
        match s {
            "ent_seq" => Tag::EntSeq,
            "keb" => Tag::Keb,
            "reb" => Tag::Reb,
            "pos" => Tag::Pos,
            "gloss" => Tag::Gloss,
            "misc" => Tag::Misc,
            "sense" => Tag::Sense,
            _ => Tag::OtherDontCareAbout,
        }
    }
}
