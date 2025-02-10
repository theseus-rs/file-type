use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_509235866: FileType = FileType {
    file_format: &FileFormat {
        id: 509_235_866,
        source_type: SourceType::Iana,
        name: "vnd.ms-wpl",
        extensions: &[],
        media_types: &["application/vnd.ms-wpl"],
        signatures: &[],
        related_formats: &[],
    },
};
