use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28052884: FileType = FileType {
    file_format: &FileFormat {
        id: 28_052_884,
        source_type: SourceType::Wikidata,
        name: "RRDC",
        extensions: &["rdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
