use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123359482: FileType = FileType {
    file_format: &FileFormat {
        id: 123_359_482,
        source_type: SourceType::Wikidata,
        name: "Personal History Project",
        extensions: &["phst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
