use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2922032133: FileType = FileType {
    file_format: &FileFormat {
        id: 2_922_032_133,
        source_type: SourceType::Iana,
        name: "feedback-report",
        extensions: &[],
        media_types: &["message/feedback-report"],
        signatures: &[],
        related_formats: &[],
    },
};
