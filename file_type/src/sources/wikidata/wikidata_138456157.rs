use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138456157: FileType = FileType {
    file_format: &FileFormat {
        id: 138_456_157,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 3.1.0",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
