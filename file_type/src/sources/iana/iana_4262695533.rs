use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4262695533: FileType = FileType {
    file_format: &FileFormat {
        id: 4_262_695_533,
        source_type: SourceType::Iana,
        name: "kb+jwt",
        extensions: &[],
        media_types: &["application/kb+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
