use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3497436453: FileFormat = FileFormat {
    id: 3_497_436_453,
    source_type: SourceType::Httpd,
    name: "anser web certificate issue initiation",
    extensions: &["cii"],
    media_types: &["application/vnd.anser-web-certificate-issue-initiation"],
    internal_signatures: &[],
    related_formats: &[],
};
