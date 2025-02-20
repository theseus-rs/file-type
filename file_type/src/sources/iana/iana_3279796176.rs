use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3279796176: FileType = FileType {
    file_format: &FileFormat {
        id: 3_279_796_176,
        source_type: SourceType::Iana,
        name: "oxps",
        extensions: &[],
        media_types: &["application/oxps"],
        signatures: &[],
        related_formats: &[],
    },
};
