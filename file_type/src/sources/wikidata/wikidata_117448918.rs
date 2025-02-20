use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117448918: FileType = FileType {
    file_format: &FileFormat {
        id: 117_448_918,
        source_type: SourceType::Wikidata,
        name: "B Source Code File",
        extensions: &["b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
