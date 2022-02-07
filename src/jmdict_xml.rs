use quick_xml::events::Event;
use quick_xml::Reader;
use regex::bytes::Regex;

use std::collections::HashMap;

pub fn process_jmdict(xml: &str) -> i32 {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut dtd_count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    let entity_re = Regex::new(r#"<!ENTITY\s+([^ \t\r\n]+)\s+"([^"]*)"\s*>"#).unwrap();
    let mut custom_entities = HashMap::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"ent_seq" => count += 1,
                _ => (),
            },
            Ok(Event::DocType(ref e)) => {
                for cap in entity_re.captures_iter(&e) {
                    custom_entities.insert(cap[1].to_vec(), cap[1].to_vec());
                }
            }
            Ok(Event::CData(ref _e)) => dtd_count += 1,
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => txt.push(
                e.unescape_and_decode_with_custom_entities(&reader, &custom_entities)
                    .expect("Could not escape and decode"),
            ),

            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

        buf.clear();
    }
    println!("count = {}", count);
    println!("txt = {:#?}", txt);
    dtd_count
}
