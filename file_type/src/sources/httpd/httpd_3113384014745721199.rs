use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3113384014745721199: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xml dtd",
    extensions: &["dtd"],
    media_types: &["application/xml-dtd"],
    internal_signatures: &[],
    related_formats: &[],
};
