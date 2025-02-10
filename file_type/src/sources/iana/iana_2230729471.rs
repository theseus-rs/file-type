use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2230729471: FileType = FileType {
    file_format: &FileFormat {
        id: 2_230_729_471,
        source_type: SourceType::Iana,
        name: "vnd.grafeq",
        extensions: &[],
        media_types: &["application/vnd.grafeq"],
        signatures: &[],
        related_formats: &[],
    },
};
