use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_623061058: FileType = FileType {
    file_format: &FileFormat {
        id: 623_061_058,
        source_type: SourceType::Iana,
        name: "mbms-protection-description+xml",
        extensions: &[],
        media_types: &["application/mbms-protection-description+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
