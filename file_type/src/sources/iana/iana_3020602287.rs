use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3020602287: FileType = FileType {
    file_format: &FileFormat {
        id: 3_020_602_287,
        source_type: SourceType::Iana,
        name: "prs.aaud",
        extensions: &[],
        media_types: &["audio/prs.aaud"],
        signatures: &[],
        related_formats: &[],
    },
};
