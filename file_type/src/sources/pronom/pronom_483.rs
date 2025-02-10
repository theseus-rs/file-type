use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_483: FileType = FileType {
    file_format: &FileFormat {
        id: 483,
        source_type: SourceType::Pronom,
        name: "Framework Database",
        extensions: &["fw", "fw2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
