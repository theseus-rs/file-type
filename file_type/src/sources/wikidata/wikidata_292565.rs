use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_292565: FileType = FileType {
    file_format: &FileFormat {
        id: 292_565,
        source_type: SourceType::Wikidata,
        name: "vCalendar",
        extensions: &["vcs"],
        media_types: &["text/x-vcalendar"],
        signatures: &[],
        related_formats: &[],
    },
};
