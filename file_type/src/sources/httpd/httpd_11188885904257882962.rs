use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11188885904257882962: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "postscript",
    extensions: &["ai", "eps", "ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
