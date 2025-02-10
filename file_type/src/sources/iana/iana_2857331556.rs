use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2857331556: FileType = FileType {
    file_format: &FileFormat {
        id: 2_857_331_556,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.template",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.template"],
        signatures: &[],
        related_formats: &[],
    },
};
