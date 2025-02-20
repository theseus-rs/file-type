use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445580: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_580,
        source_type: SourceType::Wikidata,
        name: "Application Developer Documentation Data",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
