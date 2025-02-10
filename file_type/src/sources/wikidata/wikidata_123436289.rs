use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123436289: FileType = FileType {
    file_format: &FileFormat {
        id: 123_436_289,
        source_type: SourceType::Wikidata,
        name: "DARC-F1 file",
        extensions: &["f1d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
