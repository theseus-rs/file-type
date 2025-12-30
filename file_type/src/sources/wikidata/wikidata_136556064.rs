use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136556064: FileType = FileType {
    file_format: &FileFormat {
        id: 136_556_064,
        source_type: SourceType::Wikidata,
        name: "Wii U Compressed",
        extensions: &["wux"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
