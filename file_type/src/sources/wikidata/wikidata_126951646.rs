use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126951646: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_646,
        source_type: SourceType::Wikidata,
        name: "Lex source file",
        extensions: &["l"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
