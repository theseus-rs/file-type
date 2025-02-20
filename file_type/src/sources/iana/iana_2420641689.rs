use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2420641689: FileType = FileType {
    file_format: &FileFormat {
        id: 2_420_641_689,
        source_type: SourceType::Iana,
        name: "vnd.c3voc.schedule+xml",
        extensions: &[],
        media_types: &["application/vnd.c3voc.schedule+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
