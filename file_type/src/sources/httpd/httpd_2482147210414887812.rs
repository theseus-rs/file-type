use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2482147210414887812: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hyperstudio",
    extensions: &["stk"],
    media_types: &["application/hyperstudio"],
    internal_signatures: &[],
    related_formats: &[],
};
