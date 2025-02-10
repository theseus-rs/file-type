use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2390: FileType = FileType {
    file_format: &FileFormat {
        id: 2_390,
        source_type: SourceType::Pronom,
        name: "reStructuredText",
        extensions: &["rst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
