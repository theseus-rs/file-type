use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_510862015: FileType = FileType {
    file_format: &FileFormat {
        id: 510_862_015,
        source_type: SourceType::Iana,
        name: "step+xml",
        extensions: &[],
        media_types: &["model/step+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
