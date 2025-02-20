use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131332032: FileType = FileType {
    file_format: &FileFormat {
        id: 131_332_032,
        source_type: SourceType::Wikidata,
        name: "TypoScript code",
        extensions: &["typoscript"],
        media_types: &["text/x-typoscript"],
        signatures: &[],
        related_formats: &[],
    },
};
