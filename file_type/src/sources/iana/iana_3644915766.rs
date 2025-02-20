use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3644915766: FileType = FileType {
    file_format: &FileFormat {
        id: 3_644_915_766,
        source_type: SourceType::Iana,
        name: "nv",
        extensions: &[],
        media_types: &["video/nv"],
        signatures: &[],
        related_formats: &[],
    },
};
