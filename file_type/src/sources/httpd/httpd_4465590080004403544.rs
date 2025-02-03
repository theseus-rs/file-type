use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4465590080004403544: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "csh",
    extensions: &["csh"],
    media_types: &["application/x-csh"],
    internal_signatures: &[],
    related_formats: &[],
};
