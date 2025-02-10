use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119494021: FileType = FileType {
    file_format: &FileFormat {
        id: 119_494_021,
        source_type: SourceType::Wikidata,
        name: "SnagIt Capture File",
        extensions: &["snag"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
