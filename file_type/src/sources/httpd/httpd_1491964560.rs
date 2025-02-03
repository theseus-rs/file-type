use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1491964560: FileFormat = FileFormat {
    id: 1_491_964_560,
    source_type: SourceType::Httpd,
    name: "nzb",
    extensions: &["nzb"],
    media_types: &["application/x-nzb"],
    internal_signatures: &[],
    related_formats: &[],
};
