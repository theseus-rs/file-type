use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_207819: FileType = FileType {
    file_format: &FileFormat {
        id: 207_819,
        source_type: SourceType::Wikidata,
        name: "Standard Generalized Markup Language",
        extensions: &["sgml"],
        media_types: &["application/sgml", "text/sgml"],
        signatures: &[],
        related_formats: &[],
    },
};
