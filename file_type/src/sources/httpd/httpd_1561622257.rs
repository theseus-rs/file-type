use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1561622257: FileType = FileType {
    file_format: &FileFormat {
        id: 1_561_622_257,
        source_type: SourceType::Httpd,
        name: "msaccess",
        extensions: &["mdb"],
        media_types: &["application/x-msaccess"],
        signatures: &[],
        related_formats: &[],
    },
};
