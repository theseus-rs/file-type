use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7128109369329973531: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "svd",
    extensions: &["svd"],
    media_types: &["application/vnd.svd"],
    internal_signatures: &[],
    related_formats: &[],
};
