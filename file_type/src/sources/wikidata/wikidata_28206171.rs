use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206171: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_171,
        source_type: SourceType::Wikidata,
        name: "GIMP Animated Brush",
        extensions: &["gih"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
