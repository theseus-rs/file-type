use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9678658877874429298: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "pn realaudio",
    extensions: &["ram", "ra"],
    media_types: &["audio/x-pn-realaudio"],
    internal_signatures: &[],
    related_formats: &[],
};
