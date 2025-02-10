use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1041686558: FileType = FileType {
    file_format: &FileFormat {
        id: 1_041_686_558,
        source_type: SourceType::Iana,
        name: "webpush-options+json",
        extensions: &[],
        media_types: &["application/webpush-options+json"],
        signatures: &[],
        related_formats: &[],
    },
};
