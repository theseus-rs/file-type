use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206450: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_450,
        source_type: SourceType::Wikidata,
        name: "KiSS CEL 8-bit",
        extensions: &["cel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
