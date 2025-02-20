use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3504459710: FileType = FileType {
    file_format: &FileFormat {
        id: 3_504_459_710,
        source_type: SourceType::Iana,
        name: "logout+jwt",
        extensions: &[],
        media_types: &["application/logout+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
