use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4000171426298219438: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xpinstall",
    extensions: &["xpi"],
    media_types: &["application/x-xpinstall"],
    internal_signatures: &[],
    related_formats: &[],
};
