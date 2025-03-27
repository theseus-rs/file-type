use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944773: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_773,
        source_type: SourceType::Wikidata,
        name: "Rich Text Format, version 1.1",
        extensions: &["rtf"],
        media_types: &["application/rtf", "text/rtf"],
        signatures: &[],
        related_formats: &[],
    },
};
