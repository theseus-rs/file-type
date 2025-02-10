use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99973606: FileType = FileType {
    file_format: &FileFormat {
        id: 99_973_606,
        source_type: SourceType::Wikidata,
        name: "XDOMEA 2.0.0",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
