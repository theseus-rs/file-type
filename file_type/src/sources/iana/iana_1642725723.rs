use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1642725723: FileType = FileType {
    file_format: &FileFormat {
        id: 1_642_725_723,
        source_type: SourceType::Iana,
        name: "CEA",
        extensions: &[],
        media_types: &["application/CEA"],
        signatures: &[],
        related_formats: &[],
    },
};
