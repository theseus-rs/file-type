use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114237069: FileType = FileType {
    file_format: &FileFormat {
        id: 114_237_069,
        source_type: SourceType::Wikidata,
        name: "Visual C++ Definition format",
        extensions: &["def"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
