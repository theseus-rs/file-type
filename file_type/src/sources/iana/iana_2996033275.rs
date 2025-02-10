use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2996033275: FileType = FileType {
    file_format: &FileFormat {
        id: 2_996_033_275,
        source_type: SourceType::Iana,
        name: "vnd.pagerduty+json",
        extensions: &[],
        media_types: &["application/vnd.pagerduty+json"],
        signatures: &[],
        related_formats: &[],
    },
};
