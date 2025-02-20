use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445585: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_585,
        source_type: SourceType::Wikidata,
        name: "Application Label Index",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
