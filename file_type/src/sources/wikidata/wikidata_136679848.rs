use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136679848: FileType = FileType {
    file_format: &FileFormat {
        id: 136_679_848,
        source_type: SourceType::Wikidata,
        name: "Photobook Designer file",
        extensions: &["dp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
