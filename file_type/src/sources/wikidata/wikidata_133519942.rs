use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133519942: FileType = FileType {
    file_format: &FileFormat {
        id: 133_519_942,
        source_type: SourceType::Wikidata,
        name: "Dali medium resolution file",
        extensions: &["mpk", "sd1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
