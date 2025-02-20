use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3278359110: FileType = FileType {
    file_format: &FileFormat {
        id: 3_278_359_110,
        source_type: SourceType::Httpd,
        name: "fdf",
        extensions: &["fdf"],
        media_types: &["application/vnd.fdf"],
        signatures: &[],
        related_formats: &[],
    },
};
