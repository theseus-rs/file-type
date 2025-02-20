use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127824045: FileType = FileType {
    file_format: &FileFormat {
        id: 127_824_045,
        source_type: SourceType::Wikidata,
        name: "Cinema DTS Audio file format",
        extensions: &["apx", "aud", "aue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
