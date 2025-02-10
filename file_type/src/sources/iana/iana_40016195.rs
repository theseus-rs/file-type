use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_40016195: FileType = FileType {
    file_format: &FileFormat {
        id: 40_016_195,
        source_type: SourceType::Iana,
        name: "activity+json",
        extensions: &[],
        media_types: &["application/activity+json"],
        signatures: &[],
        related_formats: &[],
    },
};
