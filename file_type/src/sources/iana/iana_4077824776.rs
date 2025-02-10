use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4077824776: FileType = FileType {
    file_format: &FileFormat {
        id: 4_077_824_776,
        source_type: SourceType::Iana,
        name: "vnd.commonspace",
        extensions: &[],
        media_types: &["application/vnd.commonspace"],
        signatures: &[],
        related_formats: &[],
    },
};
