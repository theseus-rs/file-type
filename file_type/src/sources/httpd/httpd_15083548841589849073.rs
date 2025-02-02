use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15083548841589849073: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "t3vm image",
    extensions: &["t3"],
    media_types: &["application/x-t3vm-image"],
    internal_signatures: &[],
    related_formats: &[],
};
