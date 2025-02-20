use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_219983: FileType = FileType {
    file_format: &FileFormat {
        id: 219_983,
        source_type: SourceType::Wikidata,
        name: "Zoo",
        extensions: &["zoo"],
        media_types: &["application/x-zoo"],
        signatures: &[],
        related_formats: &[],
    },
};
