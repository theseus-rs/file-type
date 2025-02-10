use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1655456439: FileType = FileType {
    file_format: &FileFormat {
        id: 1_655_456_439,
        source_type: SourceType::Iana,
        name: "news (OBSOLETED by [RFC5537])",
        extensions: &[],
        media_types: &["message/news"],
        signatures: &[],
        related_formats: &[],
    },
};
