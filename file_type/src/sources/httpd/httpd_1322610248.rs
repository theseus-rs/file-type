use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1322610248: FileType = FileType {
    file_format: &FileFormat {
        id: 1_322_610_248,
        source_type: SourceType::Httpd,
        name: "fortran",
        extensions: &["f", "for", "f77", "f90"],
        media_types: &["text/x-fortran"],
        signatures: &[],
        related_formats: &[],
    },
};
