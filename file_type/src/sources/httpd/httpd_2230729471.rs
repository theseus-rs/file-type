use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2230729471: FileType = FileType {
    file_format: &FileFormat {
        id: 2_230_729_471,
        source_type: SourceType::Httpd,
        name: "grafeq",
        extensions: &["gqf", "gqs"],
        media_types: &["application/vnd.grafeq"],
        signatures: &[],
        related_formats: &[],
    },
};
