use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138324875: FileType = FileType {
    file_format: &FileFormat {
        id: 138_324_875,
        source_type: SourceType::Wikidata,
        name: "POWDER",
        extensions: &["xml"],
        media_types: &["application/powder+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
