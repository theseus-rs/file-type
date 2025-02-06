use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_284651: FileFormat = FileFormat {
    id: 284_651,
    source_type: SourceType::Wikidata,
    name: "iCalendar",
    extensions: &["iCal", "iFBf", "icalendar", "ics", "ifb"],
    media_types: &["text/calendar"],
    signatures: &[],
    related_formats: &[],
};
