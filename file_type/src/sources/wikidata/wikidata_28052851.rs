use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28052851: FileType = FileType {
    file_format: &FileFormat {
        id: 28_052_851,
        source_type: SourceType::Wikidata,
        name: "RePub",
        extensions: &["epub"],
        media_types: &["application/epub+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
