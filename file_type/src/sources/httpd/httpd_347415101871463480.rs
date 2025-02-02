use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_347415101871463480: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ustar",
    extensions: &["ustar"],
    media_types: &["application/x-ustar"],
    internal_signatures: &[],
    related_formats: &[],
};
