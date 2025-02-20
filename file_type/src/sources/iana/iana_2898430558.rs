use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2898430558: FileType = FileType {
    file_format: &FileFormat {
        id: 2_898_430_558,
        source_type: SourceType::Iana,
        name: "jais",
        extensions: &[],
        media_types: &["image/jais"],
        signatures: &[],
        related_formats: &[],
    },
};
