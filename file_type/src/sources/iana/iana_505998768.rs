use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_505998768: FileType = FileType {
    file_format: &FileFormat {
        id: 505_998_768,
        source_type: SourceType::Iana,
        name: "cwt",
        extensions: &[],
        media_types: &["application/cwt"],
        signatures: &[],
        related_formats: &[],
    },
};
