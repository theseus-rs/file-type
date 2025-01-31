use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_284651: FileFormat = FileFormat {
    id: 284_651,
    puid: "wikidata/284651",
    name: "iCalendar",
    extensions: &["iCal", "iFBf", "icalendar", "ics", "ifb"],
    media_types: &[
        "text/calendar",
        "text/calendar",
        "text/calendar",
        "text/calendar",
        "text/calendar",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
