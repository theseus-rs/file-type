use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944785: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_785,
        source_type: SourceType::Wikidata,
        name: "Rich Text Format, version 1.8",
        extensions: &["rtf"],
        media_types: &["application/rtf", "text/rtf"],
        signatures: &[],
        related_formats: &[],
    },
};
