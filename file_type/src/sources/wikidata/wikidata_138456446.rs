use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138456446: FileType = FileType {
    file_format: &FileFormat {
        id: 138_456_446,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 4.0.0",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
