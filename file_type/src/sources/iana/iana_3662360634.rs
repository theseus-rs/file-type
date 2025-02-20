use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3662360634: FileType = FileType {
    file_format: &FileFormat {
        id: 3_662_360_634,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
