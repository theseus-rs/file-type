use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34289890: FileType = FileType {
    file_format: &FileFormat {
        id: 34_289_890,
        source_type: SourceType::Wikidata,
        name: "SETI@Home data file",
        extensions: &["sah"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
