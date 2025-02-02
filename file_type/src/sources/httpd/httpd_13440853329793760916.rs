use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13440853329793760916: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "uuencode",
    extensions: &["uu"],
    media_types: &["text/x-uuencode"],
    internal_signatures: &[],
    related_formats: &[],
};
