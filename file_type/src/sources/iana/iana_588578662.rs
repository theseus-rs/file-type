use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_588578662: FileType = FileType {
    file_format: &FileFormat {
        id: 588_578_662,
        source_type: SourceType::Iana,
        name: "VP9",
        extensions: &[],
        media_types: &["video/VP9"],
        signatures: &[],
        related_formats: &[],
    },
};
