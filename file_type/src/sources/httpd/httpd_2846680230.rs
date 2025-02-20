use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2846680230: FileType = FileType {
    file_format: &FileFormat {
        id: 2_846_680_230,
        source_type: SourceType::Httpd,
        name: "mets xml",
        extensions: &["mets"],
        media_types: &["application/mets+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
