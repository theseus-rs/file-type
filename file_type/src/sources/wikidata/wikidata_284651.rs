use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_284651: FileFormat = FileFormat {
    id: 284_651,
    source_type: SourceType::Wikidata,
    name: "iCalendar",
    extensions: &["iCal", "iFBf", "icalendar", "ics", "ifb"],
    media_types: &["text/calendar"],
    internal_signatures: &[],
    related_formats: &[],
};
