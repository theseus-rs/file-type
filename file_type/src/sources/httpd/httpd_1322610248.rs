use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1322610248: FileFormat = FileFormat {
    id: 1_322_610_248,
    source_type: SourceType::Httpd,
    name: "fortran",
    extensions: &["f", "for", "f77", "f90"],
    media_types: &["text/x-fortran"],
    internal_signatures: &[],
    related_formats: &[],
};
