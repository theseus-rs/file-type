use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1148648632: FileType = FileType {
    file_format: &FileFormat {
        id: 1_148_648_632,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.slide+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slide+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
