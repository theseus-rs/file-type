use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1940771396: FileType = FileType {
    file_format: &FileFormat {
        id: 1_940_771_396,
        source_type: SourceType::Iana,
        name: "vnd.rar",
        extensions: &[],
        media_types: &["application/vnd.rar"],
        signatures: &[],
        related_formats: &[],
    },
};
