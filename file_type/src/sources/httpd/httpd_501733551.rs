use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_501733551: FileFormat = FileFormat {
    id: 501_733_551,
    source_type: SourceType::Httpd,
    name: "svd",
    extensions: &["svd"],
    media_types: &["application/vnd.svd"],
    internal_signatures: &[],
    related_formats: &[],
};
