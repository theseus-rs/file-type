use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_136456478: FileType = FileType {
    file_format: &FileFormat {
        id: 136_456_478,
        source_type: SourceType::Linguist,
        name: "NMODL",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
