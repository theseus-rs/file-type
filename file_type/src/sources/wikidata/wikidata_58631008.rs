use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58631008: FileType = FileType {
    file_format: &FileFormat {
        id: 58_631_008,
        source_type: SourceType::Wikidata,
        name: "Harris Matrix",
        extensions: &["hm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
