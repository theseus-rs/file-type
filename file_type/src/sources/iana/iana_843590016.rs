use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_843590016: FileType = FileType {
    file_format: &FileFormat {
        id: 843_590_016,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.document",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
