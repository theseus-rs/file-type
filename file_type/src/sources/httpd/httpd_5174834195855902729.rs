use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5174834195855902729: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dvi",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    internal_signatures: &[],
    related_formats: &[],
};
