use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1166919: FileType = FileType {
    file_format: &FileFormat {
        id: 1_166_919,
        source_type: SourceType::Wikidata,
        name: "Darwin Information Typing Architecture",
        extensions: &["dita", "xml"],
        media_types: &["application/dita+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
