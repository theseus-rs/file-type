use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1441612472: FileType = FileType {
    file_format: &FileFormat {
        id: 1_441_612_472,
        source_type: SourceType::Httpd,
        name: "eszigno3 xml",
        extensions: &["es3", "et3"],
        media_types: &["application/vnd.eszigno3+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
