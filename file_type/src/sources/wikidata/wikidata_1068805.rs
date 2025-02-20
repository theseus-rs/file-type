use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1068805: FileType = FileType {
    file_format: &FileFormat {
        id: 1_068_805,
        source_type: SourceType::Wikidata,
        name: ".properties",
        extensions: &["properties"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
