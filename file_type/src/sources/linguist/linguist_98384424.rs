use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_98384424: FileType = FileType {
    file_format: &FileFormat {
        id: 98_384_424,
        source_type: SourceType::Linguist,
        name: "iCalendar",
        extensions: &["ical", "ics"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
