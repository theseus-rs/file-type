use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4102443812: FileType = FileType {
    file_format: &FileFormat {
        id: 4_102_443_812,
        source_type: SourceType::Iana,
        name: "vnd.fints",
        extensions: &[],
        media_types: &["application/vnd.fints"],
        signatures: &[],
        related_formats: &[],
    },
};
