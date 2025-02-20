use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_284651: FileType = FileType {
    file_format: &FileFormat {
        id: 284_651,
        source_type: SourceType::Wikidata,
        name: "iCalendar",
        extensions: &["iCal", "iFBf", "icalendar", "ics", "ifb"],
        media_types: &["text/calendar"],
        signatures: &[],
        related_formats: &[],
    },
};
