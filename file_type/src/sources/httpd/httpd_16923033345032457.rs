use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16923033345032457: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "symbian install",
    extensions: &["sis", "sisx"],
    media_types: &["application/vnd.symbian.install"],
    internal_signatures: &[],
    related_formats: &[],
};
