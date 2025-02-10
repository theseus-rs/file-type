use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3110119314: FileType = FileType {
    file_format: &FileFormat {
        id: 3_110_119_314,
        source_type: SourceType::Iana,
        name: "jph",
        extensions: &[],
        media_types: &["image/jph"],
        signatures: &[],
        related_formats: &[],
    },
};
