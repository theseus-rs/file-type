use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1091729195: FileType = FileType {
    file_format: &FileFormat {
        id: 1_091_729_195,
        source_type: SourceType::Iana,
        name: "partial",
        extensions: &[],
        media_types: &["message/partial"],
        signatures: &[],
        related_formats: &[],
    },
};
