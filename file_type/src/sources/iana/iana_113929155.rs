use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_113929155: FileType = FileType {
    file_format: &FileFormat {
        id: 113_929_155,
        source_type: SourceType::Iana,
        name: "cwl",
        extensions: &[],
        media_types: &["application/cwl"],
        signatures: &[],
        related_formats: &[],
    },
};
