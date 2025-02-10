use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_567721668: FileType = FileType {
    file_format: &FileFormat {
        id: 567_721_668,
        source_type: SourceType::Iana,
        name: "vnd.microsoft.icon",
        extensions: &[],
        media_types: &["image/vnd.microsoft.icon"],
        signatures: &[],
        related_formats: &[],
    },
};
